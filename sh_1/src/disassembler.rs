pub type AddrType = u32;
macro_rules! impl_read_to_type {
    ($ unsigned_type : ty , $ signed_type : ty , $ len : literal , $ read_unsigned : ident , $ read_signed : ident , $ write_unsigned : ident , $ write_signed : ident) => {
        fn $read_unsigned<const BIG_ENDIAN: bool>(
            data: [u8; $len],
            start_bit: usize,
            len_bits: usize,
        ) -> $unsigned_type {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(TYPE_BITS / 8 == $len);
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let mut data = if BIG_ENDIAN {
                <$unsigned_type>::from_be_bytes(data)
            } else {
                <$unsigned_type>::from_le_bytes(data)
            };
            let value_mask = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            data = data >> start_bit;
            data = data & value_mask;
            data
        }
        fn $read_signed<const BIG_ENDIAN: bool>(
            data: [u8; $len],
            start_bit: usize,
            len_bits: usize,
        ) -> $signed_type {
            const TYPE_BITS: usize = <$signed_type>::BITS as usize;
            assert!(len_bits > 1);
            assert!(TYPE_BITS / 8 == $len);
            let data = $read_unsigned::<BIG_ENDIAN>(data, start_bit, len_bits);
            let value_mask = <$unsigned_type>::try_from(<$signed_type>::MAX)
                .unwrap()
                >> (TYPE_BITS - len_bits);
            let sign_mask = !value_mask;
            let value_part = data & value_mask;
            let sign_part = data & sign_mask;
            if sign_part != 0 {
                let neg_value = (!value_part + 1) & value_mask;
                <$signed_type>::try_from(neg_value)
                    .unwrap()
                    .checked_neg()
                    .unwrap()
            } else {
                <$signed_type>::try_from(value_part).unwrap()
            }
        }
        fn $write_unsigned<const BIG_ENDIAN: bool>(
            value: $unsigned_type,
            mem: $unsigned_type,
            start_bit: usize,
            len_bits: usize,
        ) -> [u8; $len] {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let value_max = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            let mask = value_max << start_bit;
            let mut value = value;
            value <<= start_bit;
            value = (mem & !mask) | value;
            if BIG_ENDIAN {
                value.to_be_bytes()
            } else {
                value.to_le_bytes()
            }
        }
        fn $write_signed<const BIG_ENDIAN: bool>(
            value: $signed_type,
            mem: $signed_type,
            start_bit: usize,
            len_bits: usize,
        ) -> [u8; $len] {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let value: $unsigned_type = if value < 0 {
                <$unsigned_type>::MAX
                    - <$unsigned_type>::try_from(value.abs() - 1).unwrap()
            } else {
                <$unsigned_type>::try_from(value).unwrap()
            };
            let mem: $unsigned_type = if mem < 0 {
                <$unsigned_type>::MAX
                    - <$unsigned_type>::try_from(mem.abs() - 1).unwrap()
            } else {
                <$unsigned_type>::try_from(value).unwrap()
            };
            let mask = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            let value = value & mask;
            $write_unsigned::<BIG_ENDIAN>(value, mem, start_bit, len_bits)
        }
    };
}
impl_read_to_type!(u8, i8, 1, read_u8, read_i8, write_u8, write_i8);
impl_read_to_type!(u16, i16, 2, read_u16, read_i16, write_u16, write_i16);
impl_read_to_type!(u32, i32, 4, read_u32, read_i32, write_u32, write_i32);
impl_read_to_type!(u64, i64, 8, read_u64, read_i64, write_u64, write_i64);
impl_read_to_type!(
    u128, i128, 16, read_u128, read_i128, write_u128, write_i128
);
impl_read_to_type!(
    ethnum::u256,
    ethnum::i256,
    32,
    read_u256,
    read_i256,
    write_u256,
    write_i256
);
pub trait GlobalSetTrait {}
pub trait MemoryRead {
    type AddressType;
    fn read(&self, addr: Self::AddressType, buf: &mut [u8]);
}
pub trait MemoryWrite {
    type AddressType;
    fn write(&mut self, addr: Self::AddressType, buf: &[u8]);
}
pub trait ContextTrait {}
#[derive(Debug, Clone, Copy, Default)]
pub struct SpacesStruct {}
impl ContextTrait for SpacesStruct {}
fn meaning_number<T>(hex: bool, num: T) -> DisplayElement
where
    i64: TryFrom<T>,
    <i64 as TryFrom<T>>::Error: core::fmt::Debug,
{
    DisplayElement::Number(hex, i64::try_from(num).unwrap())
}
fn meaning_0_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_0_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_0_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::r0,
        1 => Register::r1,
        2 => Register::r2,
        3 => Register::r3,
        4 => Register::r4,
        5 => Register::r5,
        6 => Register::r6,
        7 => Register::r7,
        8 => Register::r8,
        9 => Register::r9,
        10 => Register::r10,
        11 => Register::r11,
        12 => Register::r12,
        13 => Register::r13,
        14 => Register::r14,
        15 => Register::r15,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_1_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => DisplayElement::Literal("r0"),
        1 => DisplayElement::Literal("r1"),
        2 => DisplayElement::Literal("r2"),
        3 => DisplayElement::Literal("r3"),
        4 => DisplayElement::Literal("r4"),
        5 => DisplayElement::Literal("r5"),
        6 => DisplayElement::Literal("r6"),
        7 => DisplayElement::Literal("r7"),
        8 => DisplayElement::Literal("r8"),
        9 => DisplayElement::Literal("r9"),
        10 => DisplayElement::Literal("r10"),
        11 => DisplayElement::Literal("r11"),
        12 => DisplayElement::Literal("r12"),
        13 => DisplayElement::Literal("r13"),
        14 => DisplayElement::Literal("r14"),
        15 => DisplayElement::Literal("pr"),
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_disp_00_03(u8);
impl TokenField_disp_00_03 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_sdisp_00_03(i8);
impl TokenField_sdisp_00_03 {
    fn execution(&self) -> i8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_disp_00_07(u8);
impl TokenField_disp_00_07 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_sdisp_00_07(i8);
impl TokenField_sdisp_00_07 {
    fn execution(&self) -> i8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_disp_00_11(u16);
impl TokenField_disp_00_11 {
    fn execution(&self) -> u16 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_sdisp_00_11(i16);
impl TokenField_sdisp_00_11 {
    fn execution(&self) -> i16 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_imm3_00_02(u8);
impl TokenField_imm3_00_02 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_imm_00_07(u8);
impl TokenField_imm_00_07 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_simm_00_07(i8);
impl TokenField_simm_00_07 {
    fn execution(&self) -> i8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_00_03(u8);
impl TokenField_opcode_00_03 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_00_07(u8);
impl TokenField_opcode_00_07 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_00_15(u16);
impl TokenField_opcode_00_15 {
    fn execution(&self) -> u16 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_03_03(u8);
impl TokenField_opcode_03_03 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_04_07(u8);
impl TokenField_opcode_04_07 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_08_11(u8);
impl TokenField_opcode_08_11 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_08_15(u8);
impl TokenField_opcode_08_15 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opcode_12_15(u8);
impl TokenField_opcode_12_15 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_rm_04_07(u8);
impl TokenField_rm_04_07 {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_rm_08_11(u8);
impl TokenField_rm_08_11 {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_rn_04_07(u8);
impl TokenField_rn_04_07 {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_rn_08_11(u8);
impl TokenField_rn_08_11 {
    fn execution(&self) -> Register {
        meaning_0_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_0_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_rm_imm_08_11(u8);
impl TokenField_rm_imm_08_11 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_1_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_rn_imm_08_11(u8);
impl TokenField_rn_imm_08_11 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_1_display(self.0)
    }
}
struct TokenParser<const LEN: usize>([u8; LEN]);
impl<const LEN: usize> TokenParser<LEN> {
    fn new(data: &[u8]) -> Option<Self> {
        let token_slice: &[u8] = data.get(..LEN)?;
        let token_data = <[u8; LEN]>::try_from(token_slice).unwrap();
        Some(Self(token_data))
    }
    fn TokenFielddisp_00_03(&self) -> TokenField_disp_00_03 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_disp_00_03(inner_value)
    }
    fn TokenFieldsdisp_00_03(&self) -> TokenField_sdisp_00_03 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_sdisp_00_03(inner_value)
    }
    fn TokenFielddisp_00_07(&self) -> TokenField_disp_00_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_disp_00_07(inner_value)
    }
    fn TokenFieldsdisp_00_07(&self) -> TokenField_sdisp_00_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_sdisp_00_07(inner_value)
    }
    fn TokenFielddisp_00_11(&self) -> TokenField_disp_00_11 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<true>(work_value, 0u64 as usize, 12u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_disp_00_11(inner_value)
    }
    fn TokenFieldsdisp_00_11(&self) -> TokenField_sdisp_00_11 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i16::<true>(work_value, 0u64 as usize, 12u64 as usize);
            i16::try_from(value).unwrap()
        };
        TokenField_sdisp_00_11(inner_value)
    }
    fn TokenFieldimm3_00_02(&self) -> TokenField_imm3_00_02 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_imm3_00_02(inner_value)
    }
    fn TokenFieldimm_00_07(&self) -> TokenField_imm_00_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_imm_00_07(inner_value)
    }
    fn TokenFieldsimm_00_07(&self) -> TokenField_simm_00_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_simm_00_07(inner_value)
    }
    fn TokenFieldopcode_00_03(&self) -> TokenField_opcode_00_03 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opcode_00_03(inner_value)
    }
    fn TokenFieldopcode_00_07(&self) -> TokenField_opcode_00_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opcode_00_07(inner_value)
    }
    fn TokenFieldopcode_00_15(&self) -> TokenField_opcode_00_15 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<true>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_opcode_00_15(inner_value)
    }
    fn TokenFieldopcode_03_03(&self) -> TokenField_opcode_03_03 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 3u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opcode_03_03(inner_value)
    }
    fn TokenFieldopcode_04_07(&self) -> TokenField_opcode_04_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 4u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opcode_04_07(inner_value)
    }
    fn TokenFieldopcode_08_11(&self) -> TokenField_opcode_08_11 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opcode_08_11(inner_value)
    }
    fn TokenFieldopcode_08_15(&self) -> TokenField_opcode_08_15 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opcode_08_15(inner_value)
    }
    fn TokenFieldopcode_12_15(&self) -> TokenField_opcode_12_15 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 4u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opcode_12_15(inner_value)
    }
    fn TokenFieldrm_04_07(&self) -> TokenField_rm_04_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 4u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rm_04_07(inner_value)
    }
    fn TokenFieldrm_08_11(&self) -> TokenField_rm_08_11 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rm_08_11(inner_value)
    }
    fn TokenFieldrn_04_07(&self) -> TokenField_rn_04_07 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 4u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rn_04_07(inner_value)
    }
    fn TokenFieldrn_08_11(&self) -> TokenField_rn_08_11 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rn_08_11(inner_value)
    }
    fn TokenFieldrm_imm_08_11(&self) -> TokenField_rm_imm_08_11 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rm_imm_08_11(inner_value)
    }
    fn TokenFieldrn_imm_08_11(&self) -> TokenField_rn_imm_08_11 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rn_imm_08_11(inner_value)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
    sr,
    gbr,
    vbr,
    mach,
    macl,
    pr,
    pc,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::r0 => write!(f, "r0"),
            Self::r1 => write!(f, "r1"),
            Self::r2 => write!(f, "r2"),
            Self::r3 => write!(f, "r3"),
            Self::r4 => write!(f, "r4"),
            Self::r5 => write!(f, "r5"),
            Self::r6 => write!(f, "r6"),
            Self::r7 => write!(f, "r7"),
            Self::r8 => write!(f, "r8"),
            Self::r9 => write!(f, "r9"),
            Self::r10 => write!(f, "r10"),
            Self::r11 => write!(f, "r11"),
            Self::r12 => write!(f, "r12"),
            Self::r13 => write!(f, "r13"),
            Self::r14 => write!(f, "r14"),
            Self::r15 => write!(f, "r15"),
            Self::sr => write!(f, "sr"),
            Self::gbr => write!(f, "gbr"),
            Self::vbr => write!(f, "vbr"),
            Self::mach => write!(f, "mach"),
            Self::macl => write!(f, "macl"),
            Self::pr => write!(f, "pr"),
            Self::pc => write!(f, "pc"),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum DisplayElement {
    Literal(&'static str),
    Register(Register),
    Number(bool, i64),
}
impl core::fmt::Display for DisplayElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(lit) => lit.fmt(f),
            Self::Register(reg) => reg.fmt(f),
            Self::Number(hex, value) => match (*hex, value.is_negative()) {
                (true, true) => write!(f, "-0x{:x}", value.abs()),
                (true, false) => write!(f, "0x{:x}", value),
                (false, _) => value.fmt(f),
            },
        }
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1011:1"]
#[derive(Clone, Debug)]
struct instructionVar0 {}
impl instructionVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("div0u")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 25i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1685:1"]
#[derive(Clone, Debug)]
struct instructionVar1 {}
impl instructionVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("rts")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 11i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1731:1"]
#[derive(Clone, Debug)]
struct instructionVar2 {}
impl instructionVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("clrmac")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 40i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1737:1"]
#[derive(Clone, Debug)]
struct instructionVar3 {}
impl instructionVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("clrt")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 8i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1860:1"]
#[derive(Clone, Debug)]
struct instructionVar4 {}
impl instructionVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nop")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 9i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1866:1"]
#[derive(Clone, Debug)]
struct instructionVar5 {}
impl instructionVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("rte")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 43i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1882:1"]
#[derive(Clone, Debug)]
struct instructionVar6 {}
impl instructionVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("sett")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 24i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1889:1"]
#[derive(Clone, Debug)]
struct instructionVar7 {}
impl instructionVar7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("sleep")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 27i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:245:1"]
#[derive(Clone, Debug)]
struct instructionVar8 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal("+,"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 4i64 {
            return None;
        }
        let mut sub_pattern_c100 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldopcode_04_07().disassembly()
                != token_parser.TokenFieldopcode_08_11().disassembly()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c100(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:257:1"]
#[derive(Clone, Debug)]
struct instructionVar9 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal("+,"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 5i64 {
            return None;
        }
        let mut sub_pattern_c100 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldopcode_04_07().disassembly()
                != token_parser.TokenFieldopcode_08_11().disassembly()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c100(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:269:1"]
#[derive(Clone, Debug)]
struct instructionVar10 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal("+,"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 6i64 {
            return None;
        }
        let mut sub_pattern_c100 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldopcode_04_07().disassembly()
                != token_parser.TokenFieldopcode_08_11().disassembly()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c100(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:388:1"]
#[derive(Clone, Debug)]
struct instructionVar11 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("movt"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 41i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:935:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/pl"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 21i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:940:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/pz"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 17i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1413:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar14 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("tas.b"),
            DisplayElement::Literal("  @"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 27i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1465:1"]
#[derive(Clone, Debug)]
struct instructionVar15 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("rotcl"),
            DisplayElement::Literal("  "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 36i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1476:1"]
#[derive(Clone, Debug)]
struct instructionVar16 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("rotcr"),
            DisplayElement::Literal("  "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 37i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1487:1"]
#[derive(Clone, Debug)]
struct instructionVar17 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar17 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("rotl"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 4i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1494:1"]
#[derive(Clone, Debug)]
struct instructionVar18 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar18 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("rotr"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 5i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1518:1"]
#[derive(Clone, Debug)]
struct instructionVar19 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar19 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shal"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 32i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1526:1"]
#[derive(Clone, Debug)]
struct instructionVar20 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar20 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shar"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 33i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1550:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar21 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shll"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 0i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1558:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar22 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shll2"),
            DisplayElement::Literal("  "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 8i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1563:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar23 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shll8"),
            DisplayElement::Literal("  "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 24i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1568:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar24 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shll16"),
            DisplayElement::Literal(" "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 40i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1573:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar25 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shlr"),
            DisplayElement::Literal("   "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 1i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1581:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar26 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shlr2"),
            DisplayElement::Literal("  "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 9i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1586:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar27 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shlr8"),
            DisplayElement::Literal("  "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 25i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1591:1"]
#[derive(Clone, Debug)]
struct instructionVar28 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar28 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("shlr16"),
            DisplayElement::Literal(" "),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 41i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1665:1"]
#[derive(Clone, Debug)]
struct instructionVar29 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar29 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("jmp"),
            DisplayElement::Literal("    @"),
            self.rm_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 43i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1673:1"]
#[derive(Clone, Debug)]
struct instructionVar30 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar30 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("jsr"),
            DisplayElement::Literal("    @"),
            self.rm_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 11i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1775:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar31 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ldc"),
            DisplayElement::Literal("    "),
            self.rm_08_11.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::sr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 14i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1780:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar32 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal("  @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::sr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 7i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1787:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar33 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ldc"),
            DisplayElement::Literal("    "),
            self.rm_08_11.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 30i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1799:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar34 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal("  @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::gbr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 23i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1805:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar35 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ldc"),
            DisplayElement::Literal("    "),
            self.rm_08_11.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::vbr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 46i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1810:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar36 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal("  @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::vbr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 39i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1816:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar37 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lds"),
            DisplayElement::Literal("    "),
            self.rm_08_11.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::mach),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 10i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1826:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar38 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal("  @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::mach),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 6i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1838:1"]
#[derive(Clone, Debug)]
struct instructionVar39 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar39 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lds"),
            DisplayElement::Literal("    "),
            self.rm_08_11.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::macl),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 26i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1843:1"]
#[derive(Clone, Debug)]
struct instructionVar40 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar40 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal("  @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::macl),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 22i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1849:1"]
#[derive(Clone, Debug)]
struct instructionVar41 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar41 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lds"),
            DisplayElement::Literal("    "),
            self.rm_08_11.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::pr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 42i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1854:1"]
#[derive(Clone, Debug)]
struct instructionVar42 {
    rm_08_11: TokenField_rm_08_11,
}
impl instructionVar42 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal("  @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::pr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 38i64 {
            return None;
        }
        let rm_08_11 = token_parser.TokenFieldrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1894:1"]
#[derive(Clone, Debug)]
struct instructionVar43 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar43 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("stc"),
            DisplayElement::Literal("    "),
            DisplayElement::Register(Register::sr),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 2i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1899:1"]
#[derive(Clone, Debug)]
struct instructionVar44 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar44 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::sr),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 3i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1905:1"]
#[derive(Clone, Debug)]
struct instructionVar45 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar45 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("stc"),
            DisplayElement::Literal("    "),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 18i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1917:1"]
#[derive(Clone, Debug)]
struct instructionVar46 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar46 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 19i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1923:1"]
#[derive(Clone, Debug)]
struct instructionVar47 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar47 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("stc"),
            DisplayElement::Literal("    "),
            DisplayElement::Register(Register::vbr),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 34i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1928:1"]
#[derive(Clone, Debug)]
struct instructionVar48 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar48 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::vbr),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 35i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1934:1"]
#[derive(Clone, Debug)]
struct instructionVar49 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar49 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sts"),
            DisplayElement::Literal("    "),
            DisplayElement::Register(Register::mach),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 10i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1944:1"]
#[derive(Clone, Debug)]
struct instructionVar50 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar50 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::mach),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 2i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1958:1"]
#[derive(Clone, Debug)]
struct instructionVar51 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar51 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sts"),
            DisplayElement::Literal("    "),
            DisplayElement::Register(Register::macl),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 26i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1963:1"]
#[derive(Clone, Debug)]
struct instructionVar52 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar52 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::macl),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 18i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1969:1"]
#[derive(Clone, Debug)]
struct instructionVar53 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar53 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sts"),
            DisplayElement::Literal("    "),
            DisplayElement::Register(Register::pr),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 42i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1974:1"]
#[derive(Clone, Debug)]
struct instructionVar54 {
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar54 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::pr),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 34i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:179:1"]
#[derive(Clone, Debug)]
struct instructionVar55 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar55 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 3i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:199:1"]
#[derive(Clone, Debug)]
struct instructionVar56 {
    disppc4: Tabledisppc4,
}
impl instructionVar56 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mova"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.disppc4.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 199i64 {
            return None;
        }
        let disppc4 = if let Some((len, table)) = Tabledisppc4::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disppc4 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:214:1"]
#[derive(Clone, Debug)]
struct instructionVar57 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar57 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 0i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:219:1"]
#[derive(Clone, Debug)]
struct instructionVar58 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar58 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 1i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:224:1"]
#[derive(Clone, Debug)]
struct instructionVar59 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar59 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 2i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:229:1"]
#[derive(Clone, Debug)]
struct instructionVar60 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar60 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 0i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:234:1"]
#[derive(Clone, Debug)]
struct instructionVar61 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar61 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 1i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:239:1"]
#[derive(Clone, Debug)]
struct instructionVar62 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar62 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 2i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:250:1"]
#[derive(Clone, Debug)]
struct instructionVar63 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar63 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal("+,"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 4i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:262:1"]
#[derive(Clone, Debug)]
struct instructionVar64 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar64 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal("+,"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 5i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        let opcode_04_07 = token_parser.TokenFieldopcode_04_07();
        let opcode_08_11 = token_parser.TokenFieldopcode_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:274:1"]
#[derive(Clone, Debug)]
struct instructionVar65 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar65 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal("+,"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 6i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        let opcode_04_07 = token_parser.TokenFieldopcode_04_07();
        let opcode_08_11 = token_parser.TokenFieldopcode_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:280:1"]
#[derive(Clone, Debug)]
struct instructionVar66 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar66 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 4i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:286:1"]
#[derive(Clone, Debug)]
struct instructionVar67 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar67 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 5i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:292:1"]
#[derive(Clone, Debug)]
struct instructionVar68 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar68 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@-"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 6i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:298:1"]
#[derive(Clone, Debug)]
struct instructionVar69 {
    disp_00_03: TokenField_disp_00_03,
    rm_04_07: TokenField_rm_04_07,
}
impl instructionVar69 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  @("),
            self.disp_00_03.display(),
            DisplayElement::Literal(","),
            self.rm_04_07.display(),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 132i64 {
            return None;
        }
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        let disp_00_03 = token_parser.TokenFielddisp_00_03();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rm_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:303:1"]
#[derive(Clone, Debug)]
struct instructionVar70 {
    rm_04_07: TokenField_rm_04_07,
    disp_00_03: TokenField_disp_00_03,
}
impl instructionVar70 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_03
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.rm_04_07.display(),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 133i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_03()
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        let disp_00_03 = token_parser.TokenFielddisp_00_03();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rm_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:313:1"]
#[derive(Clone, Debug)]
struct instructionVar71 {
    disp_00_03: TokenField_disp_00_03,
    rn_04_07: TokenField_rn_04_07,
}
impl instructionVar71 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(",@("),
            self.disp_00_03.display(),
            DisplayElement::Literal(","),
            self.rn_04_07.display(),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 128i64 {
            return None;
        }
        let rn_04_07 = token_parser.TokenFieldrn_04_07();
        let disp_00_03 = token_parser.TokenFielddisp_00_03();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:318:1"]
#[derive(Clone, Debug)]
struct instructionVar72 {
    rn_04_07: TokenField_rn_04_07,
    disp_00_03: TokenField_disp_00_03,
}
impl instructionVar72 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_03
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(",@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.rn_04_07.display(),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 129i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_03()
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let rn_04_07 = token_parser.TokenFieldrn_04_07();
        let disp_00_03 = token_parser.TokenFielddisp_00_03();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:328:1"]
#[derive(Clone, Debug)]
struct instructionVar73 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar73 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  @("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.rm_04_07.display(),
            DisplayElement::Literal("),"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 12i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:333:1"]
#[derive(Clone, Debug)]
struct instructionVar74 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar74 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  @("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.rm_04_07.display(),
            DisplayElement::Literal("),"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 13i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:338:1"]
#[derive(Clone, Debug)]
struct instructionVar75 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar75 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  @("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.rm_04_07.display(),
            DisplayElement::Literal("),"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 14i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:343:1"]
#[derive(Clone, Debug)]
struct instructionVar76 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar76 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 4i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:348:1"]
#[derive(Clone, Debug)]
struct instructionVar77 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar77 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 5i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:353:1"]
#[derive(Clone, Debug)]
struct instructionVar78 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar78 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 6i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:358:1"]
#[derive(Clone, Debug)]
struct instructionVar79 {
    disp_00_07: TokenField_disp_00_07,
}
impl instructionVar79 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  @("),
            self.disp_00_07.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 196i64 {
            return None;
        }
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:363:1"]
#[derive(Clone, Debug)]
struct instructionVar80 {
    disp_00_07: TokenField_disp_00_07,
}
impl instructionVar80 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_07
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 197i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_07()
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:368:1"]
#[derive(Clone, Debug)]
struct instructionVar81 {
    disp_00_07: TokenField_disp_00_07,
}
impl instructionVar81 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_07
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 198i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_07()
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:373:1"]
#[derive(Clone, Debug)]
struct instructionVar82 {
    disp_00_07: TokenField_disp_00_07,
}
impl instructionVar82 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(",@("),
            self.disp_00_07.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 192i64 {
            return None;
        }
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:378:1"]
#[derive(Clone, Debug)]
struct instructionVar83 {
    disp_00_07: TokenField_disp_00_07,
}
impl instructionVar83 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_07
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(",@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 193i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_07()
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0);
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:383:1"]
#[derive(Clone, Debug)]
struct instructionVar84 {
    disp_00_07: TokenField_disp_00_07,
}
impl instructionVar84 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_07
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(",@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 194i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_07()
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:828:1"]
#[derive(Clone, Debug)]
struct instructionVar85 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar85 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("swap.b"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 8i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:840:1"]
#[derive(Clone, Debug)]
struct instructionVar86 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar86 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("swap.w"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 9i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:850:1"]
#[derive(Clone, Debug)]
struct instructionVar87 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar87 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("xtrct"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 13i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:863:1"]
#[derive(Clone, Debug)]
struct instructionVar88 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar88 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("add"),
            DisplayElement::Literal("    "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 12i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:873:1"]
#[derive(Clone, Debug)]
struct instructionVar89 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar89 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("addc"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 14i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:886:1"]
#[derive(Clone, Debug)]
struct instructionVar90 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar90 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("addv"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 15i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:905:1"]
#[derive(Clone, Debug)]
struct instructionVar91 {
    simm_00_07: TokenField_simm_00_07,
}
impl instructionVar91 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal("   "),
            self.simm_00_07.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 136i64 {
            return None;
        }
        let simm_00_07 = token_parser.TokenFieldsimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:910:1"]
#[derive(Clone, Debug)]
struct instructionVar92 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar92 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 0i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:915:1"]
#[derive(Clone, Debug)]
struct instructionVar93 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar93 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/hs"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 2i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:920:1"]
#[derive(Clone, Debug)]
struct instructionVar94 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar94 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/ge"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 3i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:925:1"]
#[derive(Clone, Debug)]
struct instructionVar95 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar95 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/hi"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 6i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:930:1"]
#[derive(Clone, Debug)]
struct instructionVar96 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar96 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/gt"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 7i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:945:1"]
#[derive(Clone, Debug)]
struct instructionVar97 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar97 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/str"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 12i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1003:1"]
#[derive(Clone, Debug)]
struct instructionVar98 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar98 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("div0s"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 7i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1018:1"]
#[derive(Clone, Debug)]
struct instructionVar99 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar99 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("div1"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 4i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1088:1"]
#[derive(Clone, Debug)]
struct instructionVar100 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar100 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("exts.b"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 14i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1094:1"]
#[derive(Clone, Debug)]
struct instructionVar101 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar101 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("exts.w"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 15i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1100:1"]
#[derive(Clone, Debug)]
struct instructionVar102 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar102 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("extu.b"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 12i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1105:1"]
#[derive(Clone, Debug)]
struct instructionVar103 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar103 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("extu.w"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 13i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1220:1"]
#[derive(Clone, Debug)]
struct instructionVar104 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar104 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("mac.w"),
            DisplayElement::Literal("  @"),
            self.rm_04_07.display(),
            DisplayElement::Literal("+,@"),
            self.rn_08_11.display(),
            DisplayElement::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 15i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1306:1"]
#[derive(Clone, Debug)]
struct instructionVar105 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar105 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("muls.w"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 15i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1311:1"]
#[derive(Clone, Debug)]
struct instructionVar106 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar106 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mulu.w"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 14i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1316:1"]
#[derive(Clone, Debug)]
struct instructionVar107 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar107 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("neg"),
            DisplayElement::Literal("    "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 11i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1321:1"]
#[derive(Clone, Debug)]
struct instructionVar108 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar108 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("negc"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 10i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1333:1"]
#[derive(Clone, Debug)]
struct instructionVar109 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar109 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sub"),
            DisplayElement::Literal("    "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 8i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1338:1"]
#[derive(Clone, Debug)]
struct instructionVar110 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar110 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("subc"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 10i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1351:1"]
#[derive(Clone, Debug)]
struct instructionVar111 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar111 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("subv"),
            DisplayElement::Literal("   "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 11i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1372:1"]
#[derive(Clone, Debug)]
struct instructionVar112 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar112 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("and"),
            DisplayElement::Literal("    "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 9i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1377:1"]
#[derive(Clone, Debug)]
struct instructionVar113 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar113 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("and"),
            DisplayElement::Literal("   "),
            self.imm_00_07.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 201i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1382:1"]
#[derive(Clone, Debug)]
struct instructionVar114 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar114 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("and.b"),
            DisplayElement::Literal("  "),
            self.imm_00_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 205i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1390:1"]
#[derive(Clone, Debug)]
struct instructionVar115 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar115 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("not"),
            DisplayElement::Literal("    "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 7i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1395:1"]
#[derive(Clone, Debug)]
struct instructionVar116 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar116 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("or"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 11i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1400:1"]
#[derive(Clone, Debug)]
struct instructionVar117 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar117 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("or"),
            DisplayElement::Literal(" "),
            self.imm_00_07.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 203i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1405:1"]
#[derive(Clone, Debug)]
struct instructionVar118 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar118 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("or.b"),
            DisplayElement::Literal("  "),
            self.imm_00_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 207i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1424:1"]
#[derive(Clone, Debug)]
struct instructionVar119 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar119 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("tst"),
            DisplayElement::Literal("    "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 8i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1429:1"]
#[derive(Clone, Debug)]
struct instructionVar120 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar120 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("tst"),
            DisplayElement::Literal("    "),
            self.imm_00_07.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 200i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1436:1"]
#[derive(Clone, Debug)]
struct instructionVar121 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar121 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("tst.b"),
            DisplayElement::Literal("  "),
            self.imm_00_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 204i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1444:1"]
#[derive(Clone, Debug)]
struct instructionVar122 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar122 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("xor"),
            DisplayElement::Literal("    "),
            self.rm_04_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_03().disassembly() != 10i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11, rm_04_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1449:1"]
#[derive(Clone, Debug)]
struct instructionVar123 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar123 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("xor"),
            DisplayElement::Literal("   "),
            self.imm_00_07.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 202i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1454:1"]
#[derive(Clone, Debug)]
struct instructionVar124 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar124 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("xor.b"),
            DisplayElement::Literal("  "),
            self.imm_00_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::gbr),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 206i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1599:1"]
#[derive(Clone, Debug)]
struct instructionVar125 {
    target00_07: Tabletarget00_07,
}
impl instructionVar125 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bf"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.target00_07.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 139i64 {
            return None;
        }
        let target00_07 = if let Some((len, table)) = Tabletarget00_07::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1613:1"]
#[derive(Clone, Debug)]
struct instructionVar126 {
    target00_07: Tabletarget00_07,
}
impl instructionVar126 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bt"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.target00_07.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 137i64 {
            return None;
        }
        let target00_07 = if let Some((len, table)) = Tabletarget00_07::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1980:1"]
#[derive(Clone, Debug)]
struct instructionVar127 {
    imm_00_07: TokenField_imm_00_07,
}
impl instructionVar127 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("trapa"),
            DisplayElement::Literal("  "),
            self.imm_00_07.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_08_15().disassembly() != 195i64 {
            return None;
        }
        let imm_00_07 = token_parser.TokenFieldimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:186:1"]
#[derive(Clone, Debug)]
struct instructionVar128 {
    rn_08_11: TokenField_rn_08_11,
    imm8: Tableimm8,
}
impl instructionVar128 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("mov"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 14i64 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8, rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:204:1"]
#[derive(Clone, Debug)]
struct instructionVar129 {
    rn_08_11: TokenField_rn_08_11,
    disppc2: Tabledisppc2,
}
impl instructionVar129 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.disppc2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 9i64 {
            return None;
        }
        let disppc2 = if let Some((len, table)) = Tabledisppc2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disppc2, rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:209:1"]
#[derive(Clone, Debug)]
struct instructionVar130 {
    rn_08_11: TokenField_rn_08_11,
    disppc4: Tabledisppc4,
}
impl instructionVar130 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.disppc4.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 13i64 {
            return None;
        }
        let disppc4 = if let Some((len, table)) = Tabledisppc4::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disppc4, rn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:308:1"]
#[derive(Clone, Debug)]
struct instructionVar131 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
    disp_00_03: TokenField_disp_00_03,
}
impl instructionVar131 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_03
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.rm_04_07.display(),
            DisplayElement::Literal("),"),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 5i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_03()
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        let disp_00_03 = token_parser.TokenFielddisp_00_03();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_08_11,
                rm_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:323:1"]
#[derive(Clone, Debug)]
struct instructionVar132 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
    disp_00_03: TokenField_disp_00_03,
}
impl instructionVar132 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_03
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 1i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_03()
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0);
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let rm_04_07 = token_parser.TokenFieldrm_04_07();
        let disp_00_03 = token_parser.TokenFielddisp_00_03();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_08_11,
                rm_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:868:1"]
#[derive(Clone, Debug)]
struct instructionVar133 {
    simm_00_07: TokenField_simm_00_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar133 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("add"),
            DisplayElement::Literal("    "),
            self.simm_00_07.display(),
            DisplayElement::Literal(","),
            self.rn_08_11.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 7i64 {
            return None;
        }
        let rn_08_11 = token_parser.TokenFieldrn_08_11();
        let simm_00_07 = token_parser.TokenFieldsimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_08_11,
                simm_00_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1627:1"]
#[derive(Clone, Debug)]
struct instructionVar134 {
    target00_11: Tabletarget00_11,
}
impl instructionVar134 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("bra"),
            DisplayElement::Literal("    "),
        ];
        display.extend_from_slice(&extend);
        self.target00_11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 10i64 {
            return None;
        }
        let target00_11 = if let Some((len, table)) = Tabletarget00_11::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1642:1"]
#[derive(Clone, Debug)]
struct instructionVar135 {
    target00_11: Tabletarget00_11,
}
impl instructionVar135 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("bsr"),
            DisplayElement::Literal("    "),
        ];
        display.extend_from_slice(&extend);
        self.target00_11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_12_15().disassembly() != 11i64 {
            return None;
        }
        let target00_11 = if let Some((len, table)) = Tabletarget00_11::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_11 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(instructionVar0),
    Var1(instructionVar1),
    Var2(instructionVar2),
    Var3(instructionVar3),
    Var4(instructionVar4),
    Var5(instructionVar5),
    Var6(instructionVar6),
    Var7(instructionVar7),
    Var8(instructionVar8),
    Var9(instructionVar9),
    Var10(instructionVar10),
    Var11(instructionVar11),
    Var12(instructionVar12),
    Var13(instructionVar13),
    Var14(instructionVar14),
    Var15(instructionVar15),
    Var16(instructionVar16),
    Var17(instructionVar17),
    Var18(instructionVar18),
    Var19(instructionVar19),
    Var20(instructionVar20),
    Var21(instructionVar21),
    Var22(instructionVar22),
    Var23(instructionVar23),
    Var24(instructionVar24),
    Var25(instructionVar25),
    Var26(instructionVar26),
    Var27(instructionVar27),
    Var28(instructionVar28),
    Var29(instructionVar29),
    Var30(instructionVar30),
    Var31(instructionVar31),
    Var32(instructionVar32),
    Var33(instructionVar33),
    Var34(instructionVar34),
    Var35(instructionVar35),
    Var36(instructionVar36),
    Var37(instructionVar37),
    Var38(instructionVar38),
    Var39(instructionVar39),
    Var40(instructionVar40),
    Var41(instructionVar41),
    Var42(instructionVar42),
    Var43(instructionVar43),
    Var44(instructionVar44),
    Var45(instructionVar45),
    Var46(instructionVar46),
    Var47(instructionVar47),
    Var48(instructionVar48),
    Var49(instructionVar49),
    Var50(instructionVar50),
    Var51(instructionVar51),
    Var52(instructionVar52),
    Var53(instructionVar53),
    Var54(instructionVar54),
    Var55(instructionVar55),
    Var56(instructionVar56),
    Var57(instructionVar57),
    Var58(instructionVar58),
    Var59(instructionVar59),
    Var60(instructionVar60),
    Var61(instructionVar61),
    Var62(instructionVar62),
    Var63(instructionVar63),
    Var64(instructionVar64),
    Var65(instructionVar65),
    Var66(instructionVar66),
    Var67(instructionVar67),
    Var68(instructionVar68),
    Var69(instructionVar69),
    Var70(instructionVar70),
    Var71(instructionVar71),
    Var72(instructionVar72),
    Var73(instructionVar73),
    Var74(instructionVar74),
    Var75(instructionVar75),
    Var76(instructionVar76),
    Var77(instructionVar77),
    Var78(instructionVar78),
    Var79(instructionVar79),
    Var80(instructionVar80),
    Var81(instructionVar81),
    Var82(instructionVar82),
    Var83(instructionVar83),
    Var84(instructionVar84),
    Var85(instructionVar85),
    Var86(instructionVar86),
    Var87(instructionVar87),
    Var88(instructionVar88),
    Var89(instructionVar89),
    Var90(instructionVar90),
    Var91(instructionVar91),
    Var92(instructionVar92),
    Var93(instructionVar93),
    Var94(instructionVar94),
    Var95(instructionVar95),
    Var96(instructionVar96),
    Var97(instructionVar97),
    Var98(instructionVar98),
    Var99(instructionVar99),
    Var100(instructionVar100),
    Var101(instructionVar101),
    Var102(instructionVar102),
    Var103(instructionVar103),
    Var104(instructionVar104),
    Var105(instructionVar105),
    Var106(instructionVar106),
    Var107(instructionVar107),
    Var108(instructionVar108),
    Var109(instructionVar109),
    Var110(instructionVar110),
    Var111(instructionVar111),
    Var112(instructionVar112),
    Var113(instructionVar113),
    Var114(instructionVar114),
    Var115(instructionVar115),
    Var116(instructionVar116),
    Var117(instructionVar117),
    Var118(instructionVar118),
    Var119(instructionVar119),
    Var120(instructionVar120),
    Var121(instructionVar121),
    Var122(instructionVar122),
    Var123(instructionVar123),
    Var124(instructionVar124),
    Var125(instructionVar125),
    Var126(instructionVar126),
    Var127(instructionVar127),
    Var128(instructionVar128),
    Var129(instructionVar129),
    Var130(instructionVar130),
    Var131(instructionVar131),
    Var132(instructionVar132),
    Var133(instructionVar133),
    Var134(instructionVar134),
    Var135(instructionVar135),
}
impl Tableinstruction {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var3(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var4(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var5(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var6(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var7(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var8(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var9(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var10(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var11(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var12(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var13(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var14(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var15(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var16(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var17(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var18(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var19(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var20(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var21(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var22(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var23(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var24(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var25(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var26(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var27(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var28(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var29(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var30(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var31(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var32(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var33(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var34(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var35(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var36(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var37(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var38(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var39(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var40(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var41(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var42(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var43(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var44(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var45(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var46(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var47(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var48(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var49(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var50(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var51(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var52(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var53(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var54(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var55(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var56(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var57(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var58(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var59(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var60(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var61(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var62(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var63(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var64(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var65(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var66(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var67(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var68(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var69(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var70(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var71(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var72(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var73(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var74(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var75(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var76(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var77(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var78(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var79(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var80(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var81(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var82(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var83(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var84(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var85(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var86(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var87(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var88(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var89(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var90(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var91(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var92(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var93(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var94(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var95(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var96(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var97(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var98(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var99(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var100(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var101(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var102(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var103(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var104(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var105(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var106(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var107(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var108(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var109(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var110(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var111(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var112(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var113(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var114(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var115(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var116(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var117(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var118(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var119(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var120(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var121(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var122(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var123(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var124(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var125(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var126(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var127(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var128(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var129(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var130(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var131(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var132(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var133(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var134(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var135(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var93(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var94(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var95(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var96(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var97(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var122(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var123(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var124(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var125(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var126(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var127(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var128(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var129(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var130(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var131(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var132(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var133(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var134(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var135(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:166:1"]
#[derive(Clone, Debug)]
struct target00_07Var0 {
    sdisp_00_07: TokenField_sdisp_00_07,
}
impl target00_07Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_target: i64 = 0;
        calc_target = self
            .sdisp_00_07
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(i64::try_from(inst_start).unwrap())
            .wrapping_add(4i64);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_target)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_target: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_target = token_parser
            .TokenFieldsdisp_00_07()
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(i64::try_from(inst_start).unwrap())
            .wrapping_add(4i64);
        let sdisp_00_07 = token_parser.TokenFieldsdisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sdisp_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tabletarget00_07 {
    Var0(target00_07Var0),
}
impl Tabletarget00_07 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = target00_07Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:170:1"]
#[derive(Clone, Debug)]
struct target00_11Var0 {
    sdisp_00_11: TokenField_sdisp_00_11,
}
impl target00_11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_target: i64 = 0;
        calc_target = self
            .sdisp_00_11
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(i64::try_from(inst_start).unwrap())
            .wrapping_add(4i64);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_target)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_target: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_target = token_parser
            .TokenFieldsdisp_00_11()
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(i64::try_from(inst_start).unwrap())
            .wrapping_add(4i64);
        let sdisp_00_11 = token_parser.TokenFieldsdisp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sdisp_00_11 }))
    }
}
#[derive(Clone, Debug)]
enum Tabletarget00_11 {
    Var0(target00_11Var0),
}
impl Tabletarget00_11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = target00_11Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:184:1"]
#[derive(Clone, Debug)]
struct imm8Var0 {
    simm_00_07: TokenField_simm_00_07,
}
impl imm8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("#"), self.simm_00_07.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let simm_00_07 = token_parser.TokenFieldsimm_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimm8 {
    Var0(imm8Var0),
}
impl Tableimm8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            imm8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:191:1"]
#[derive(Clone, Debug)]
struct disppc4Var0 {
    disp_00_07: TokenField_disp_00_07,
}
impl disppc4Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_07
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(
                (i64::try_from(inst_start).unwrap().wrapping_add(4i64)
                    & 4294967292i64),
            );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::pc),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFielddisp_00_07()
            .disassembly()
            .checked_shl(u32::try_from(2i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(
                (i64::try_from(inst_start).unwrap().wrapping_add(4i64)
                    & 4294967292i64),
            );
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tabledisppc4 {
    Var0(disppc4Var0),
}
impl Tabledisppc4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            disppc4Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:195:1"]
#[derive(Clone, Debug)]
struct disppc2Var0 {
    disp_00_07: TokenField_disp_00_07,
}
impl disppc2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i64 = 0;
        calc_disp = self
            .disp_00_07
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(
                i64::try_from(inst_start).unwrap().wrapping_add(4i64),
            );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::pc),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_disp: i64 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFielddisp_00_07()
            .disassembly()
            .checked_shl(u32::try_from(1i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(
                i64::try_from(inst_start).unwrap().wrapping_add(4i64),
            );
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tabledisppc2 {
    Var0(disppc2Var0),
}
impl Tabledisppc2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            disppc2Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
pub fn parse_instruction<T>(
    tokens: &[u8],
    context: &mut T,
    inst_start: u32,
    global_set: &mut impl GlobalSetTrait,
) -> Option<(u32, Vec<DisplayElement>)>
where
    T: ContextTrait + Clone,
{
    let (inst_len, instruction) =
        Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(
        &mut display,
        context,
        inst_start,
        inst_next,
        global_set,
    );
    Some((inst_next, display))
}
