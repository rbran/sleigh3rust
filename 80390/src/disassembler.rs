pub type AddrType = u32;
macro_rules! impl_read_to_type {
    ($ unsigned_type : ty , $ signed_type : ty , $ len : literal , $ read_unsigned : ident , $ read_signed : ident , $ write_unsigned : ident , $ write_signed : ident) => {
        const fn $read_unsigned<const BIG_ENDIAN: bool>(
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
        const fn $read_signed<const BIG_ENDIAN: bool>(
            data: [u8; $len],
            start_bit: usize,
            len_bits: usize,
        ) -> $signed_type {
            const TYPE_BITS: usize = <$signed_type>::BITS as usize;
            assert!(len_bits > 1);
            assert!(TYPE_BITS / 8 == $len);
            let data = $read_unsigned::<BIG_ENDIAN>(data, start_bit, len_bits);
            let value_mask =
                <$signed_type>::MAX as $unsigned_type >> (TYPE_BITS - len_bits);
            let sign_mask = !value_mask;
            let value_part = data & value_mask;
            let sign_part = data & sign_mask;
            if sign_part != 0 {
                sign_mask as $signed_type | value_part as $signed_type
            } else {
                data as $signed_type
            }
        }
        const fn $write_unsigned<const BIG_ENDIAN: bool>(
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
        const fn $write_signed<const BIG_ENDIAN: bool>(
            value: $signed_type,
            mem: $signed_type,
            start_bit: usize,
            len_bits: usize,
        ) -> [u8; $len] {
            const TYPE_BITS: usize = <$unsigned_type>::BITS as usize;
            assert!(len_bits > 0);
            assert!(len_bits + start_bit <= TYPE_BITS);
            let value_max = <$signed_type>::MAX >> (TYPE_BITS - len_bits);
            let value_min = <$signed_type>::MIN >> (TYPE_BITS - len_bits);
            let mask = <$unsigned_type>::MAX >> (TYPE_BITS - len_bits);
            let value = value as $unsigned_type & mask;
            let mem = mem as $unsigned_type;
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
        0 => Register::R0,
        1 => Register::R1,
        2 => Register::R2,
        3 => Register::R3,
        4 => Register::R4,
        5 => Register::R5,
        6 => Register::R6,
        7 => Register::R7,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_1_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_1_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_1_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R0,
        1 => Register::R1,
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_opfull(u8);
impl TokenField_opfull {
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
struct TokenField_oplo(u8);
impl TokenField_oplo {
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
struct TokenField_ophi(u8);
impl TokenField_ophi {
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
struct TokenField_rn(u8);
impl TokenField_rn {
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
struct TokenField_rnfill(u8);
impl TokenField_rnfill {
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
struct TokenField_ri(u8);
impl TokenField_ri {
    fn execution(&self) -> Register {
        meaning_1_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_1_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_rifill(u8);
impl TokenField_rifill {
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
struct TokenField_opaddr(u8);
impl TokenField_opaddr {
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
struct TokenField_addrfill(u8);
impl TokenField_addrfill {
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
struct TokenField_b_0000(u8);
impl TokenField_b_0000 {
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
struct TokenField_b_0001(u8);
impl TokenField_b_0001 {
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
struct TokenField_b_0002(u8);
impl TokenField_b_0002 {
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
struct TokenField_b_0005(u8);
impl TokenField_b_0005 {
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
struct TokenField_b_0101(u8);
impl TokenField_b_0101 {
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
struct TokenField_b_0107(u8);
impl TokenField_b_0107 {
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
struct TokenField_b_0207(u8);
impl TokenField_b_0207 {
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
struct TokenField_b_0307(u8);
impl TokenField_b_0307 {
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
struct TokenField_b_0607(u8);
impl TokenField_b_0607 {
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
struct TokenField_direct(u8);
impl TokenField_direct {
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
struct TokenField_bank(u8);
impl TokenField_bank {
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
struct TokenField_sfr(u8);
impl TokenField_sfr {
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
struct TokenField_sfr6(u8);
impl TokenField_sfr6 {
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
struct TokenField_sfrlo(u8);
impl TokenField_sfrlo {
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
struct TokenField_mainreg(u8);
impl TokenField_mainreg {
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
struct TokenField_direct17(u8);
impl TokenField_direct17 {
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
struct TokenField_direct2(u8);
impl TokenField_direct2 {
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
struct TokenField_bank2(u8);
impl TokenField_bank2 {
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
struct TokenField_sfr2(u8);
impl TokenField_sfr2 {
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
struct TokenField_sfr26(u8);
impl TokenField_sfr26 {
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
struct TokenField_sfr2lo(u8);
impl TokenField_sfr2lo {
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
struct TokenField_mainreg2(u8);
impl TokenField_mainreg2 {
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
struct TokenField_bitaddr8(u8);
impl TokenField_bitaddr8 {
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
struct TokenField_bitaddr27(u8);
impl TokenField_bitaddr27 {
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
struct TokenField_bitbank(u8);
impl TokenField_bitbank {
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
struct TokenField_sfrbyte(u8);
impl TokenField_sfrbyte {
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
struct TokenField_bitaddr57(u8);
impl TokenField_bitaddr57 {
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
struct TokenField_sfrbit6(u8);
impl TokenField_sfrbit6 {
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
struct TokenField_sfrbit3(u8);
impl TokenField_sfrbit3 {
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
struct TokenField_sfrbit(u8);
impl TokenField_sfrbit {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(false, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_lowbyte(u8);
impl TokenField_lowbyte {
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
struct TokenField_bitaddr0(u8);
impl TokenField_bitaddr0 {
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
struct TokenField_addr16(u16);
impl TokenField_addr16 {
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
struct TokenField_addr24(u32);
impl TokenField_addr24 {
    fn execution(&self) -> u32 {
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
struct TokenField_rel8(i8);
impl TokenField_rel8 {
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
struct TokenField_data(u8);
impl TokenField_data {
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
struct TokenField_data16(u16);
impl TokenField_data16 {
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
struct TokenField_rel16(i16);
impl TokenField_rel16 {
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
struct TokenField_data24(u32);
impl TokenField_data24 {
    fn execution(&self) -> u32 {
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
struct TokenField_aoplo(u8);
impl TokenField_aoplo {
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
struct TokenField_aopaddr(u8);
impl TokenField_aopaddr {
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
struct TokenField_aaddrfill(u8);
impl TokenField_aaddrfill {
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
struct TokenField_adata(u16);
impl TokenField_adata {
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
struct TokenParser<const LEN: usize>([u8; LEN]);
impl<const LEN: usize> TokenParser<LEN> {
    fn new(data: &[u8]) -> Option<Self> {
        let token_slice: &[u8] = data.get(..LEN)?;
        let token_data = <[u8; LEN]>::try_from(token_slice).unwrap();
        Some(Self(token_data))
    }
    fn TokenFieldopfull(&self) -> TokenField_opfull {
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
        TokenField_opfull(inner_value)
    }
    fn TokenFieldoplo(&self) -> TokenField_oplo {
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
        TokenField_oplo(inner_value)
    }
    fn TokenFieldophi(&self) -> TokenField_ophi {
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
        TokenField_ophi(inner_value)
    }
    fn TokenFieldrn(&self) -> TokenField_rn {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rn(inner_value)
    }
    fn TokenFieldrnfill(&self) -> TokenField_rnfill {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 3u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rnfill(inner_value)
    }
    fn TokenFieldri(&self) -> TokenField_ri {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_ri(inner_value)
    }
    fn TokenFieldrifill(&self) -> TokenField_rifill {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 1u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_rifill(inner_value)
    }
    fn TokenFieldopaddr(&self) -> TokenField_opaddr {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 5u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_opaddr(inner_value)
    }
    fn TokenFieldaddrfill(&self) -> TokenField_addrfill {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 4u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_addrfill(inner_value)
    }
    fn TokenFieldb_0000(&self) -> TokenField_b_0000 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0000(inner_value)
    }
    fn TokenFieldb_0001(&self) -> TokenField_b_0001 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0001(inner_value)
    }
    fn TokenFieldb_0002(&self) -> TokenField_b_0002 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0002(inner_value)
    }
    fn TokenFieldb_0005(&self) -> TokenField_b_0005 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0005(inner_value)
    }
    fn TokenFieldb_0101(&self) -> TokenField_b_0101 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 1u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0101(inner_value)
    }
    fn TokenFieldb_0107(&self) -> TokenField_b_0107 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 1u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0107(inner_value)
    }
    fn TokenFieldb_0207(&self) -> TokenField_b_0207 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 2u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0207(inner_value)
    }
    fn TokenFieldb_0307(&self) -> TokenField_b_0307 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0307(inner_value)
    }
    fn TokenFieldb_0607(&self) -> TokenField_b_0607 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 6u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_b_0607(inner_value)
    }
    fn TokenFielddirect(&self) -> TokenField_direct {
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
        TokenField_direct(inner_value)
    }
    fn TokenFieldbank(&self) -> TokenField_bank {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_bank(inner_value)
    }
    fn TokenFieldsfr(&self) -> TokenField_sfr {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfr(inner_value)
    }
    fn TokenFieldsfr6(&self) -> TokenField_sfr6 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 6u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfr6(inner_value)
    }
    fn TokenFieldsfrlo(&self) -> TokenField_sfrlo {
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
        TokenField_sfrlo(inner_value)
    }
    fn TokenFieldmainreg(&self) -> TokenField_mainreg {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_mainreg(inner_value)
    }
    fn TokenFielddirect17(&self) -> TokenField_direct17 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 1u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_direct17(inner_value)
    }
    fn TokenFielddirect2(&self) -> TokenField_direct2 {
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
        TokenField_direct2(inner_value)
    }
    fn TokenFieldbank2(&self) -> TokenField_bank2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_bank2(inner_value)
    }
    fn TokenFieldsfr2(&self) -> TokenField_sfr2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfr2(inner_value)
    }
    fn TokenFieldsfr26(&self) -> TokenField_sfr26 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 6u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfr26(inner_value)
    }
    fn TokenFieldsfr2lo(&self) -> TokenField_sfr2lo {
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
        TokenField_sfr2lo(inner_value)
    }
    fn TokenFieldmainreg2(&self) -> TokenField_mainreg2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_mainreg2(inner_value)
    }
    fn TokenFieldbitaddr8(&self) -> TokenField_bitaddr8 {
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
        TokenField_bitaddr8(inner_value)
    }
    fn TokenFieldbitaddr27(&self) -> TokenField_bitaddr27 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 2u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_bitaddr27(inner_value)
    }
    fn TokenFieldbitbank(&self) -> TokenField_bitbank {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_bitbank(inner_value)
    }
    fn TokenFieldsfrbyte(&self) -> TokenField_sfrbyte {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfrbyte(inner_value)
    }
    fn TokenFieldbitaddr57(&self) -> TokenField_bitaddr57 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 5u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_bitaddr57(inner_value)
    }
    fn TokenFieldsfrbit6(&self) -> TokenField_sfrbit6 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 6u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfrbit6(inner_value)
    }
    fn TokenFieldsfrbit3(&self) -> TokenField_sfrbit3 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 3u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfrbit3(inner_value)
    }
    fn TokenFieldsfrbit(&self) -> TokenField_sfrbit {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_sfrbit(inner_value)
    }
    fn TokenFieldlowbyte(&self) -> TokenField_lowbyte {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 3u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_lowbyte(inner_value)
    }
    fn TokenFieldbitaddr0(&self) -> TokenField_bitaddr0 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_bitaddr0(inner_value)
    }
    fn TokenFieldaddr16(&self) -> TokenField_addr16 {
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
        TokenField_addr16(inner_value)
    }
    fn TokenFieldaddr24(&self) -> TokenField_addr24 {
        let inner_value = {
            let mut work_value = [0u8; 4u64 as usize];
            let work_start = 1u64 as usize;
            let work_end = 4u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 3u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u32::<true>(work_value, 0u64 as usize, 24u64 as usize);
            u32::try_from(value).unwrap()
        };
        TokenField_addr24(inner_value)
    }
    fn TokenFieldrel8(&self) -> TokenField_rel8 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<true>(work_value, 0u64 as usize, 8u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_rel8(inner_value)
    }
    fn TokenFielddata(&self) -> TokenField_data {
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
        TokenField_data(inner_value)
    }
    fn TokenFielddata16(&self) -> TokenField_data16 {
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
        TokenField_data16(inner_value)
    }
    fn TokenFieldrel16(&self) -> TokenField_rel16 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i16::<true>(work_value, 0u64 as usize, 16u64 as usize);
            i16::try_from(value).unwrap()
        };
        TokenField_rel16(inner_value)
    }
    fn TokenFielddata24(&self) -> TokenField_data24 {
        let inner_value = {
            let mut work_value = [0u8; 4u64 as usize];
            let work_start = 1u64 as usize;
            let work_end = 4u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 3u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u32::<true>(work_value, 0u64 as usize, 24u64 as usize);
            u32::try_from(value).unwrap()
        };
        TokenField_data24(inner_value)
    }
    fn TokenFieldaoplo(&self) -> TokenField_aoplo {
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
        TokenField_aoplo(inner_value)
    }
    fn TokenFieldaopaddr(&self) -> TokenField_aopaddr {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 5u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_aopaddr(inner_value)
    }
    fn TokenFieldaaddrfill(&self) -> TokenField_aaddrfill {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<true>(work_value, 4u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_aaddrfill(inner_value)
    }
    fn TokenFieldadata(&self) -> TokenField_adata {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 3u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<true>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_adata(inner_value)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    jumpTableGuard1,
    jumpTableGuard2,
    R0R1R2R3,
    R1R2R3,
    R2R1,
    R0R1,
    R2R3,
    R4R5,
    R6R7,
    R4R5R6R7,
    R5R6R7,
    B,
    ACC,
    AB,
    DPTR,
    DPX,
    DPH,
    DPL,
    SP,
    PC,
    PSW,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R0 => write!(f, "R0"),
            Self::R1 => write!(f, "R1"),
            Self::R2 => write!(f, "R2"),
            Self::R3 => write!(f, "R3"),
            Self::R4 => write!(f, "R4"),
            Self::R5 => write!(f, "R5"),
            Self::R6 => write!(f, "R6"),
            Self::R7 => write!(f, "R7"),
            Self::jumpTableGuard1 => write!(f, "jumpTableGuard1"),
            Self::jumpTableGuard2 => write!(f, "jumpTableGuard2"),
            Self::R0R1R2R3 => write!(f, "R0R1R2R3"),
            Self::R1R2R3 => write!(f, "R1R2R3"),
            Self::R2R1 => write!(f, "R2R1"),
            Self::R0R1 => write!(f, "R0R1"),
            Self::R2R3 => write!(f, "R2R3"),
            Self::R4R5 => write!(f, "R4R5"),
            Self::R6R7 => write!(f, "R6R7"),
            Self::R4R5R6R7 => write!(f, "R4R5R6R7"),
            Self::R5R6R7 => write!(f, "R5R6R7"),
            Self::B => write!(f, "B"),
            Self::ACC => write!(f, "ACC"),
            Self::AB => write!(f, "AB"),
            Self::DPTR => write!(f, "DPTR"),
            Self::DPX => write!(f, "DPX"),
            Self::DPH => write!(f, "DPH"),
            Self::DPL => write!(f, "DPL"),
            Self::SP => write!(f, "SP"),
            Self::PC => write!(f, "PC"),
            Self::PSW => write!(f, "PSW"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:650:1"]
#[derive(Clone, Debug)]
struct instructionVar0 {
    Addr19: TableAddr19,
}
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ACALL"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Addr19.display_extend(
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
        let mut block_0_len = 3u64 as u32;
        let token_parser = <TokenParser<3usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaaddrfill().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldaoplo().disassembly() != 1i64 {
            return None;
        }
        let Addr19 = if let Some((len, table)) = TableAddr19::parse(
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
        Some((pattern_len, Self { Addr19 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:656:1"]
#[derive(Clone, Debug)]
struct instructionVar1 {
    Areg: TableAreg,
    Direct: TableDirect,
}
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ADD"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:658:1"]
#[derive(Clone, Debug)]
struct instructionVar2 {
    Areg: TableAreg,
    Data: TableData,
}
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ADD"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:661:1"]
#[derive(Clone, Debug)]
struct instructionVar3 {
    Areg: TableAreg,
    Direct: TableDirect,
}
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:663:1"]
#[derive(Clone, Debug)]
struct instructionVar4 {
    Areg: TableAreg,
    Data: TableData,
}
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:673:1"]
#[derive(Clone, Debug)]
struct instructionVar5 {
    Areg: TableAreg,
    Direct: TableDirect,
}
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:675:1"]
#[derive(Clone, Debug)]
struct instructionVar6 {
    Areg: TableAreg,
    Data: TableData,
}
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:676:1"]
#[derive(Clone, Debug)]
struct instructionVar7 {
    Areg: TableAreg,
    Direct: TableDirect,
}
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:677:1"]
#[derive(Clone, Debug)]
struct instructionVar8 {
    Direct: TableDirect,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:692:1"]
#[derive(Clone, Debug)]
struct instructionVar9 {
    Areg: TableAreg,
    Direct: TableDirect,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("CJNE"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct, Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:693:1"]
#[derive(Clone, Debug)]
struct instructionVar10 {
    Areg: TableAreg,
    Data: TableData,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("CJNE"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data, Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:697:1"]
#[derive(Clone, Debug)]
struct instructionVar11 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CLR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:698:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {
    CY: TableCY,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CLR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CY }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:710:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CPL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:711:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {
    CY: TableCY,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CPL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CY }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:722:1"]
#[derive(Clone, Debug)]
struct instructionVar15 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("DA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:724:1"]
#[derive(Clone, Debug)]
struct instructionVar16 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("DEC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:726:1"]
#[derive(Clone, Debug)]
struct instructionVar17 {
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("DEC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:729:1"]
#[derive(Clone, Debug)]
struct instructionVar18 {
    ABreg: TableABreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("DIV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ABreg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let ABreg = if let Some((len, table)) =
            TableABreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ABreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:733:1"]
#[derive(Clone, Debug)]
struct instructionVar19 {
    Direct: TableDirect,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DJNZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct, Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:735:1"]
#[derive(Clone, Debug)]
struct instructionVar20 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("INC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:737:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("INC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:747:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {
    DPTRreg: TableDPTRreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("INC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.DPTRreg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let DPTRreg = if let Some((len, table)) = TableDPTRreg::parse(
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
        Some((pattern_len, Self { DPTRreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:761:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JC"), DisplayElement::Literal("  ")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:762:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {
    ADPTR: TableADPTR,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JMP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ADPTR.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let ADPTR = if let Some((len, table)) =
            TableADPTR::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ADPTR }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:773:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JNC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:774:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JNZ"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:775:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JZ"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:778:1"]
#[derive(Clone, Debug)]
struct instructionVar28 {
    Addr24: TableAddr24,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LCALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr24.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Addr24 = if let Some((len, table)) = TableAddr24::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr24 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:779:1"]
#[derive(Clone, Debug)]
struct instructionVar29 {
    Addr24: TableAddr24,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LJMP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr24.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Addr24 = if let Some((len, table)) = TableAddr24::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr24 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:789:1"]
#[derive(Clone, Debug)]
struct instructionVar30 {
    Areg: TableAreg,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:791:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {
    Areg: TableAreg,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:795:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {
    Areg: TableAreg,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:797:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {
    Direct: TableDirect,
    Direct2: TableDirect2,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Direct2 = if let Some((len, table)) = TableDirect2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct, Direct2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:799:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {
    Direct: TableDirect,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:804:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {
    DPTRreg: TableDPTRreg,
    Data24: TableData24,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.DPTRreg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data24.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        let DPTRreg = if let Some((len, table)) = TableDPTRreg::parse(
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
        let mut block_1_len = 0u64 as u32;
        let Data24 = if let Some((len, table)) = TableData24::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DPTRreg, Data24 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:822:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {
    ADPTR: TableADPTR,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVC"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.ADPTR.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let ADPTR = if let Some((len, table)) =
            TableADPTR::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ADPTR, Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:823:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {
    APC: TableAPC,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVC"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.APC.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let APC = if let Some((len, table)) =
            TableAPC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { APC, Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:826:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {
    Areg: TableAreg,
    ATDPTR: TableATDPTR,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.ATDPTR.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let ATDPTR = if let Some((len, table)) = TableATDPTR::parse(
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
        Some((pattern_len, Self { Areg, ATDPTR }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:828:1"]
#[derive(Clone, Debug)]
struct instructionVar39 {
    Areg: TableAreg,
    ATDPTR: TableATDPTR,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.ATDPTR.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let ATDPTR = if let Some((len, table)) = TableATDPTR::parse(
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
        Some((pattern_len, Self { Areg, ATDPTR }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:830:1"]
#[derive(Clone, Debug)]
struct instructionVar40 {
    ABreg: TableABreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MUL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ABreg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let ABreg = if let Some((len, table)) =
            TableABreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ABreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:833:1"]
#[derive(Clone, Debug)]
struct instructionVar41 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NOP")];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:836:1"]
#[derive(Clone, Debug)]
struct instructionVar42 {
    Areg: TableAreg,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:838:1"]
#[derive(Clone, Debug)]
struct instructionVar43 {
    Areg: TableAreg,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:839:1"]
#[derive(Clone, Debug)]
struct instructionVar44 {
    Areg: TableAreg,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:840:1"]
#[derive(Clone, Debug)]
struct instructionVar45 {
    Areg: TableAreg,
    Direct: TableDirect,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:855:1"]
#[derive(Clone, Debug)]
struct instructionVar46 {
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("POP"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:857:1"]
#[derive(Clone, Debug)]
struct instructionVar47 {
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("PUSH"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:859:1"]
#[derive(Clone, Debug)]
struct instructionVar48 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RET")];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:869:1"]
#[derive(Clone, Debug)]
struct instructionVar49 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("RETI")];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:879:1"]
#[derive(Clone, Debug)]
struct instructionVar50 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("RL"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:880:1"]
#[derive(Clone, Debug)]
struct instructionVar51 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("RLC"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:881:1"]
#[derive(Clone, Debug)]
struct instructionVar52 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("RR"),
            DisplayElement::Literal("   "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:882:1"]
#[derive(Clone, Debug)]
struct instructionVar53 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("RRC"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:884:1"]
#[derive(Clone, Debug)]
struct instructionVar54 {
    CY: TableCY,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SETB"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CY }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:895:1"]
#[derive(Clone, Debug)]
struct instructionVar55 {
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SJMP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:898:1"]
#[derive(Clone, Debug)]
struct instructionVar56 {
    Areg: TableAreg,
    Direct: TableDirect,
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
            DisplayElement::Literal("SUBB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:900:1"]
#[derive(Clone, Debug)]
struct instructionVar57 {
    Areg: TableAreg,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SUBB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:902:1"]
#[derive(Clone, Debug)]
struct instructionVar58 {
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SWAP"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:905:1"]
#[derive(Clone, Debug)]
struct instructionVar59 {
    Areg: TableAreg,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XCH"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:912:1"]
#[derive(Clone, Debug)]
struct instructionVar60 {
    Areg: TableAreg,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:914:1"]
#[derive(Clone, Debug)]
struct instructionVar61 {
    Areg: TableAreg,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:915:1"]
#[derive(Clone, Debug)]
struct instructionVar62 {
    Areg: TableAreg,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:916:1"]
#[derive(Clone, Debug)]
struct instructionVar63 {
    Direct: TableDirect,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:657:1"]
#[derive(Clone, Debug)]
struct instructionVar64 {
    Areg: TableAreg,
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ADD"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:662:1"]
#[derive(Clone, Debug)]
struct instructionVar65 {
    Areg: TableAreg,
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:674:1"]
#[derive(Clone, Debug)]
struct instructionVar66 {
    Areg: TableAreg,
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:695:1"]
#[derive(Clone, Debug)]
struct instructionVar67 {
    Ri: TableRi,
    Data: TableData,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("CJNE"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri, Data, Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:727:1"]
#[derive(Clone, Debug)]
struct instructionVar68 {
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("DEC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:746:1"]
#[derive(Clone, Debug)]
struct instructionVar69 {
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("INC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:790:1"]
#[derive(Clone, Debug)]
struct instructionVar70 {
    Areg: TableAreg,
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:798:1"]
#[derive(Clone, Debug)]
struct instructionVar71 {
    Ri: TableRi,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:800:1"]
#[derive(Clone, Debug)]
struct instructionVar72 {
    Ri: TableRi,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri, Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:801:1"]
#[derive(Clone, Debug)]
struct instructionVar73 {
    Ri: TableRi,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri, Direct }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:802:1"]
#[derive(Clone, Debug)]
struct instructionVar74 {
    Ri: TableRi,
    Data: TableData,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri, Data }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:825:1"]
#[derive(Clone, Debug)]
struct instructionVar75 {
    RiX: TableRiX,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.RiX.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 1i64 {
            return None;
        }
        let RiX = if let Some((len, table)) =
            TableRiX::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RiX, Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:827:1"]
#[derive(Clone, Debug)]
struct instructionVar76 {
    RiX: TableRiX,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.RiX.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 1i64 {
            return None;
        }
        let RiX = if let Some((len, table)) =
            TableRiX::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RiX, Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:837:1"]
#[derive(Clone, Debug)]
struct instructionVar77 {
    Areg: TableAreg,
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:899:1"]
#[derive(Clone, Debug)]
struct instructionVar78 {
    Areg: TableAreg,
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SUBB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:906:1"]
#[derive(Clone, Debug)]
struct instructionVar79 {
    Ri: TableRi,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XCH"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri, Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:909:1"]
#[derive(Clone, Debug)]
struct instructionVar80 {
    Areg: TableAreg,
    Ri: TableRi,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XCHD"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, Ri }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:913:1"]
#[derive(Clone, Debug)]
struct instructionVar81 {
    Ri: TableRi,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Ri.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i64 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Ri, Areg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:655:1"]
#[derive(Clone, Debug)]
struct instructionVar82 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ADD"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:660:1"]
#[derive(Clone, Debug)]
struct instructionVar83 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:667:1"]
#[derive(Clone, Debug)]
struct instructionVar84 {
    Addr19: TableAddr19,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("AJMP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr19.display_extend(
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
        let mut block_0_len = 3u64 as u32;
        let token_parser = <TokenParser<3usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaaddrfill().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldaoplo().disassembly() != 1i64 {
            return None;
        }
        let Addr19 = if let Some((len, table)) = TableAddr19::parse(
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
        Some((pattern_len, Self { Addr19 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:672:1"]
#[derive(Clone, Debug)]
struct instructionVar85 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:679:1"]
#[derive(Clone, Debug)]
struct instructionVar86 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:680:1"]
#[derive(Clone, Debug)]
struct instructionVar87 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr2.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr2,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:700:1"]
#[derive(Clone, Debug)]
struct instructionVar88 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CLR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:713:1"]
#[derive(Clone, Debug)]
struct instructionVar89 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CPL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:749:1"]
#[derive(Clone, Debug)]
struct instructionVar90 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JB"), DisplayElement::Literal("  ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
                Rel8,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:750:1"]
#[derive(Clone, Debug)]
struct instructionVar91 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JBC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
                Rel8,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:764:1"]
#[derive(Clone, Debug)]
struct instructionVar92 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JNB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
                Rel8,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:809:1"]
#[derive(Clone, Debug)]
struct instructionVar93 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:810:1"]
#[derive(Clone, Debug)]
struct instructionVar94 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:842:1"]
#[derive(Clone, Debug)]
struct instructionVar95 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:843:1"]
#[derive(Clone, Debug)]
struct instructionVar96 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr2.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr2,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:886:1"]
#[derive(Clone, Debug)]
struct instructionVar97 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SETB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i64 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:682:1"]
#[derive(Clone, Debug)]
struct instructionVar98 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:683:1"]
#[derive(Clone, Debug)]
struct instructionVar99 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ANL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr2.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr2,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:694:1"]
#[derive(Clone, Debug)]
struct instructionVar100 {
    rn: TokenField_rn,
    Data: TableData,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("CJNE"),
            DisplayElement::Literal(" "),
            self.rn.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Data, Rel8, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:702:1"]
#[derive(Clone, Debug)]
struct instructionVar101 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CLR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:715:1"]
#[derive(Clone, Debug)]
struct instructionVar102 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("CPL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:725:1"]
#[derive(Clone, Debug)]
struct instructionVar103 {
    rn: TokenField_rn,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("DEC"),
            DisplayElement::Literal(" "),
            self.rn.display(),
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:732:1"]
#[derive(Clone, Debug)]
struct instructionVar104 {
    rn: TokenField_rn,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("DJNZ"),
            DisplayElement::Literal(" "),
            self.rn.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:736:1"]
#[derive(Clone, Debug)]
struct instructionVar105 {
    rn: TokenField_rn,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("INC"),
            DisplayElement::Literal(" "),
            self.rn.display(),
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:752:1"]
#[derive(Clone, Debug)]
struct instructionVar106 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JB"), DisplayElement::Literal("  ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
                Rel8,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:753:1"]
#[derive(Clone, Debug)]
struct instructionVar107 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JBC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
                Rel8,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:766:1"]
#[derive(Clone, Debug)]
struct instructionVar108 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JNB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u32;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
                Rel8,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:788:1"]
#[derive(Clone, Debug)]
struct instructionVar109 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:792:1"]
#[derive(Clone, Debug)]
struct instructionVar110 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("MOV"),
            DisplayElement::Literal(" "),
            self.rn.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:793:1"]
#[derive(Clone, Debug)]
struct instructionVar111 {
    rn: TokenField_rn,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("MOV"),
            DisplayElement::Literal(" "),
            self.rn.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:794:1"]
#[derive(Clone, Debug)]
struct instructionVar112 {
    rn: TokenField_rn,
    Data: TableData,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("MOV"),
            DisplayElement::Literal(" "),
            self.rn.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Data.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Data, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:796:1"]
#[derive(Clone, Debug)]
struct instructionVar113 {
    rn: TokenField_rn,
    Direct: TableDirect,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Direct, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:812:1"]
#[derive(Clone, Debug)]
struct instructionVar114 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:813:1"]
#[derive(Clone, Debug)]
struct instructionVar115 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("MOV"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:835:1"]
#[derive(Clone, Debug)]
struct instructionVar116 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:845:1"]
#[derive(Clone, Debug)]
struct instructionVar117 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:846:1"]
#[derive(Clone, Debug)]
struct instructionVar118 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ORL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BitAddr2.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i64 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                CY,
                BitAddr2,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:888:1"]
#[derive(Clone, Debug)]
struct instructionVar119 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SETB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.BitAddr.display_extend(
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                BitAddr,
                BitByteAddr,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:897:1"]
#[derive(Clone, Debug)]
struct instructionVar120 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SUBB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:904:1"]
#[derive(Clone, Debug)]
struct instructionVar121 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XCH"),
            DisplayElement::Literal("  "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:911:1"]
#[derive(Clone, Debug)]
struct instructionVar122 {
    rn: TokenField_rn,
    Areg: TableAreg,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XRL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.rn.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i64 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Areg, rn }))
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
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:507:1"]
#[derive(Clone, Debug)]
struct CYVar0 {}
impl CYVar0 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CY")];
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
        let mut block_0_len = 0u64 as u32;
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableCY {
    Var0(CYVar0),
}
impl TableCY {
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
            CYVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:509:1"]
#[derive(Clone, Debug)]
struct AregVar0 {}
impl AregVar0 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("A")];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let ophi = token_parser.TokenFieldophi();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableAreg {
    Var0(AregVar0),
}
impl TableAreg {
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
            AregVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:510:1"]
#[derive(Clone, Debug)]
struct ABregVar0 {}
impl ABregVar0 {
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
            [DisplayElement::Register(Register::AB)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let ophi = token_parser.TokenFieldophi();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableABreg {
    Var0(ABregVar0),
}
impl TableABreg {
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
            ABregVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:511:1"]
#[derive(Clone, Debug)]
struct DPTRregVar0 {}
impl DPTRregVar0 {
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
            [DisplayElement::Register(Register::DPTR)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let ophi = token_parser.TokenFieldophi();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableDPTRreg {
    Var0(DPTRregVar0),
}
impl TableDPTRreg {
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
            DPTRregVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:518:1"]
#[derive(Clone, Debug)]
struct ADPTRVar0 {}
impl ADPTRVar0 {
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
            DisplayElement::Literal("@A+"),
            DisplayElement::Register(Register::DPTR),
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let ophi = token_parser.TokenFieldophi();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableADPTR {
    Var0(ADPTRVar0),
}
impl TableADPTR {
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
            ADPTRVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:523:1"]
#[derive(Clone, Debug)]
struct APCVar0 {}
impl APCVar0 {
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
            [DisplayElement::Literal("@A+PC")];
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
        let mut block_0_len = 0u64 as u32;
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableAPC {
    Var0(APCVar0),
}
impl TableAPC {
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
            APCVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:530:1"]
#[derive(Clone, Debug)]
struct ATDPTRVar0 {}
impl ATDPTRVar0 {
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
            DisplayElement::Literal("@"),
            DisplayElement::Register(Register::DPTR),
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let ophi = token_parser.TokenFieldophi();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableATDPTR {
    Var0(ATDPTRVar0),
}
impl TableATDPTR {
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
            ATDPTRVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:540:1"]
#[derive(Clone, Debug)]
struct RiVar0 {
    ri: TokenField_ri,
}
impl RiVar0 {
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
            [DisplayElement::Literal("@"), self.ri.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let ri = token_parser.TokenFieldri();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ri }))
    }
}
#[derive(Clone, Debug)]
enum TableRi {
    Var0(RiVar0),
}
impl TableRi {
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
            RiVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:548:1"]
#[derive(Clone, Debug)]
struct RiXVar0 {
    ri: TokenField_ri,
}
impl RiXVar0 {
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
            [DisplayElement::Literal("@"), self.ri.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let ri = token_parser.TokenFieldri();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ri }))
    }
}
#[derive(Clone, Debug)]
enum TableRiX {
    Var0(RiXVar0),
}
impl TableRiX {
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
            RiXVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:553:1"]
#[derive(Clone, Debug)]
struct DataVar0 {
    data: TokenField_data,
}
impl DataVar0 {
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
            [DisplayElement::Literal("#"), self.data.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let data = token_parser.TokenFielddata();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { data }))
    }
}
#[derive(Clone, Debug)]
enum TableData {
    Var0(DataVar0),
}
impl TableData {
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
            DataVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:554:1"]
#[derive(Clone, Debug)]
struct Data16Var0 {
    data16: TokenField_data16,
}
impl Data16Var0 {
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
            [DisplayElement::Literal("#"), self.data16.display()];
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
        let data16 = token_parser.TokenFielddata16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { data16 }))
    }
}
#[derive(Clone, Debug)]
enum TableData16 {
    Var0(Data16Var0),
}
impl TableData16 {
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
            Data16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:556:1"]
#[derive(Clone, Debug)]
struct Data24Var0 {
    data24: TokenField_data24,
}
impl Data24Var0 {
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
            [DisplayElement::Literal("#"), self.data24.display()];
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
        let mut block_0_len = 3u64 as u32;
        let token_parser = <TokenParser<3usize>>::new(tokens_current)?;
        let data24 = token_parser.TokenFielddata24();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { data24 }))
    }
}
#[derive(Clone, Debug)]
enum TableData24 {
    Var0(Data24Var0),
}
impl TableData24 {
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
            Data24Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:567:1"]
#[derive(Clone, Debug)]
struct DirectVar0 {}
impl DirectVar0 {
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
            [DisplayElement::Register(Register::PSW)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 208i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:568:1"]
#[derive(Clone, Debug)]
struct DirectVar1 {}
impl DirectVar1 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("A")];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 224i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:569:1"]
#[derive(Clone, Debug)]
struct DirectVar2 {}
impl DirectVar2 {
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
            [DisplayElement::Register(Register::B)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 240i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:570:1"]
#[derive(Clone, Debug)]
struct DirectVar3 {}
impl DirectVar3 {
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
            [DisplayElement::Register(Register::DPL)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 130i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:571:1"]
#[derive(Clone, Debug)]
struct DirectVar4 {}
impl DirectVar4 {
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
            [DisplayElement::Register(Register::DPH)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 131i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:573:1"]
#[derive(Clone, Debug)]
struct DirectVar5 {}
impl DirectVar5 {
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
            [DisplayElement::Register(Register::DPX)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 147i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:564:1"]
#[derive(Clone, Debug)]
struct DirectVar6 {
    mainreg: TokenField_mainreg,
}
impl DirectVar6 {
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
        let extend: [DisplayElement; 1usize] = [self.mainreg.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 0i64 {
            return None;
        }
        let mainreg = token_parser.TokenFieldmainreg();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { mainreg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:566:1"]
#[derive(Clone, Debug)]
struct DirectVar7 {
    direct: TokenField_direct,
}
impl DirectVar7 {
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
        let extend: [DisplayElement; 1usize] = [self.direct.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i64 {
            return None;
        }
        let direct = token_parser.TokenFielddirect();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { direct }))
    }
}
#[derive(Clone, Debug)]
enum TableDirect {
    Var0(DirectVar0),
    Var1(DirectVar1),
    Var2(DirectVar2),
    Var3(DirectVar3),
    Var4(DirectVar4),
    Var5(DirectVar5),
    Var6(DirectVar6),
    Var7(DirectVar7),
}
impl TableDirect {
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
            DirectVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DirectVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DirectVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DirectVar3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DirectVar4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DirectVar5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DirectVar6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DirectVar7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:587:1"]
#[derive(Clone, Debug)]
struct Direct2Var0 {}
impl Direct2Var0 {
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
            [DisplayElement::Register(Register::PSW)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 208i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:588:1"]
#[derive(Clone, Debug)]
struct Direct2Var1 {}
impl Direct2Var1 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("A")];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 224i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:589:1"]
#[derive(Clone, Debug)]
struct Direct2Var2 {}
impl Direct2Var2 {
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
            [DisplayElement::Register(Register::B)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 240i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:590:1"]
#[derive(Clone, Debug)]
struct Direct2Var3 {}
impl Direct2Var3 {
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
            [DisplayElement::Register(Register::DPL)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 130i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:591:1"]
#[derive(Clone, Debug)]
struct Direct2Var4 {}
impl Direct2Var4 {
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
            [DisplayElement::Register(Register::DPH)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 131i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:593:1"]
#[derive(Clone, Debug)]
struct Direct2Var5 {}
impl Direct2Var5 {
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
            [DisplayElement::Register(Register::DPX)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 147i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:584:1"]
#[derive(Clone, Debug)]
struct Direct2Var6 {
    mainreg2: TokenField_mainreg2,
}
impl Direct2Var6 {
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
        let extend: [DisplayElement; 1usize] = [self.mainreg2.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 0i64 {
            return None;
        }
        let mainreg2 = token_parser.TokenFieldmainreg2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { mainreg2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:586:1"]
#[derive(Clone, Debug)]
struct Direct2Var7 {
    direct2: TokenField_direct2,
}
impl Direct2Var7 {
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
        let extend: [DisplayElement; 1usize] = [self.direct2.display()];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i64 {
            return None;
        }
        let direct2 = token_parser.TokenFielddirect2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { direct2 }))
    }
}
#[derive(Clone, Debug)]
enum TableDirect2 {
    Var0(Direct2Var0),
    Var1(Direct2Var1),
    Var2(Direct2Var2),
    Var3(Direct2Var3),
    Var4(Direct2Var4),
    Var5(Direct2Var5),
    Var6(Direct2Var6),
    Var7(Direct2Var7),
}
impl TableDirect2 {
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
            Direct2Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            Direct2Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            Direct2Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            Direct2Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            Direct2Var4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            Direct2Var5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            Direct2Var6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            Direct2Var7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:611:1"]
#[derive(Clone, Debug)]
struct BitAddrVar0 {
    sfrbyte: TokenField_sfrbyte,
    sfrbit: TokenField_sfrbit,
}
impl BitAddrVar0 {
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
        let mut bitaddr: i64 = 0;
        bitaddr = self
            .sfrbyte
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, bitaddr)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i64 {
            return None;
        }
        let mut bitaddr: i64 = 0;
        bitaddr = token_parser
            .TokenFieldsfrbyte()
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(token_parser.TokenFieldsfrbit().disassembly());
        let sfrbyte = token_parser.TokenFieldsfrbyte();
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sfrbyte, sfrbit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:612:1"]
#[derive(Clone, Debug)]
struct BitAddrVar1 {
    lowbyte: TokenField_lowbyte,
    sfrbit: TokenField_sfrbit,
}
impl BitAddrVar1 {
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
        let mut bitaddr: i64 = 0;
        bitaddr = self
            .lowbyte
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, bitaddr)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 0i64 {
            return None;
        }
        let mut bitaddr: i64 = 0;
        bitaddr = token_parser
            .TokenFieldlowbyte()
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(token_parser.TokenFieldsfrbit().disassembly());
        let lowbyte = token_parser.TokenFieldlowbyte();
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { lowbyte, sfrbit }))
    }
}
#[derive(Clone, Debug)]
enum TableBitAddr {
    Var0(BitAddrVar0),
    Var1(BitAddrVar1),
}
impl TableBitAddr {
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
            BitAddrVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            BitAddrVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:613:1"]
#[derive(Clone, Debug)]
struct BitAddr2Var0 {
    sfrbyte: TokenField_sfrbyte,
    sfrbit: TokenField_sfrbit,
}
impl BitAddr2Var0 {
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
        let mut bitaddr: i64 = 0;
        bitaddr = self
            .sfrbyte
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("/"),
            DisplayElement::Number(true, bitaddr),
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i64 {
            return None;
        }
        let mut bitaddr: i64 = 0;
        bitaddr = token_parser
            .TokenFieldsfrbyte()
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(token_parser.TokenFieldsfrbit().disassembly());
        let sfrbyte = token_parser.TokenFieldsfrbyte();
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sfrbyte, sfrbit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:614:1"]
#[derive(Clone, Debug)]
struct BitAddr2Var1 {
    lowbyte: TokenField_lowbyte,
    sfrbit: TokenField_sfrbit,
}
impl BitAddr2Var1 {
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
        let mut bitaddr: i64 = 0;
        bitaddr = self
            .lowbyte
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("/"),
            DisplayElement::Number(true, bitaddr),
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 0i64 {
            return None;
        }
        let mut bitaddr: i64 = 0;
        bitaddr = token_parser
            .TokenFieldlowbyte()
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0)
            .wrapping_add(token_parser.TokenFieldsfrbit().disassembly());
        let lowbyte = token_parser.TokenFieldlowbyte();
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { lowbyte, sfrbit }))
    }
}
#[derive(Clone, Debug)]
enum TableBitAddr2 {
    Var0(BitAddr2Var0),
    Var1(BitAddr2Var1),
}
impl TableBitAddr2 {
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
            BitAddr2Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            BitAddr2Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:618:1"]
#[derive(Clone, Debug)]
struct BitByteAddrVar0 {}
impl BitByteAddrVar0 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("A")];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbyte().disassembly() != 28i64 {
            return None;
        }
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:619:1"]
#[derive(Clone, Debug)]
struct BitByteAddrVar1 {}
impl BitByteAddrVar1 {
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
            [DisplayElement::Register(Register::B)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbyte().disassembly() != 30i64 {
            return None;
        }
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:620:1"]
#[derive(Clone, Debug)]
struct BitByteAddrVar2 {}
impl BitByteAddrVar2 {
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
            [DisplayElement::Register(Register::PSW)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldsfrbyte().disassembly() != 26i64 {
            return None;
        }
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:617:1"]
#[derive(Clone, Debug)]
struct BitByteAddrVar3 {
    sfrbyte: TokenField_sfrbyte,
}
impl BitByteAddrVar3 {
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
        let mut byteaddr: i64 = 0;
        byteaddr = self
            .sfrbyte
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, byteaddr)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i64 {
            return None;
        }
        let mut byteaddr: i64 = 0;
        byteaddr = token_parser
            .TokenFieldsfrbyte()
            .disassembly()
            .checked_shl(u32::try_from(3i64).unwrap())
            .unwrap_or(0);
        let sfrbyte = token_parser.TokenFieldsfrbyte();
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sfrbyte }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:626:1"]
#[derive(Clone, Debug)]
struct BitByteAddrVar4 {
    lowbyte: TokenField_lowbyte,
}
impl BitByteAddrVar4 {
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
        let mut byteaddr: i64 = 0;
        byteaddr = self.lowbyte.disassembly().wrapping_add(32i64);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, byteaddr)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 0i64 {
            return None;
        }
        let mut byteaddr: i64 = 0;
        byteaddr = token_parser
            .TokenFieldlowbyte()
            .disassembly()
            .wrapping_add(32i64);
        let lowbyte = token_parser.TokenFieldlowbyte();
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { lowbyte }))
    }
}
#[derive(Clone, Debug)]
enum TableBitByteAddr {
    Var0(BitByteAddrVar0),
    Var1(BitByteAddrVar1),
    Var2(BitByteAddrVar2),
    Var3(BitByteAddrVar3),
    Var4(BitByteAddrVar4),
}
impl TableBitByteAddr {
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
        if let Some((inst_len, parsed)) = BitByteAddrVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = BitByteAddrVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = BitByteAddrVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = BitByteAddrVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = BitByteAddrVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:636:1"]
#[derive(Clone, Debug)]
struct Addr19Var0 {
    aopaddr: TokenField_aopaddr,
    adata: TokenField_adata,
}
impl Addr19Var0 {
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
        let mut relAddr: i64 = 0;
        relAddr = (i64::try_from(inst_next).unwrap() & 16252928i64)
            .wrapping_add(
                self.aopaddr
                    .disassembly()
                    .wrapping_mul(256i64)
                    .wrapping_mul(256i64),
            )
            .wrapping_add(self.adata.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, relAddr)];
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
        let mut block_0_len = 3u64 as u32;
        let token_parser = <TokenParser<3usize>>::new(tokens_current)?;
        let mut relAddr: i64 = 0;
        let aopaddr = token_parser.TokenFieldaopaddr();
        let adata = token_parser.TokenFieldadata();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { aopaddr, adata }))
    }
}
#[derive(Clone, Debug)]
enum TableAddr19 {
    Var0(Addr19Var0),
}
impl TableAddr19 {
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
            Addr19Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:637:1"]
#[derive(Clone, Debug)]
struct Addr24Var0 {
    addr24: TokenField_addr24,
}
impl Addr24Var0 {
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
        let extend: [DisplayElement; 1usize] = [self.addr24.display()];
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
        let mut block_0_len = 3u64 as u32;
        let token_parser = <TokenParser<3usize>>::new(tokens_current)?;
        let addr24 = token_parser.TokenFieldaddr24();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr24 }))
    }
}
#[derive(Clone, Debug)]
enum TableAddr24 {
    Var0(Addr24Var0),
}
impl TableAddr24 {
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
            Addr24Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:640:1"]
#[derive(Clone, Debug)]
struct Rel8Var0 {
    rel8: TokenField_rel8,
}
impl Rel8Var0 {
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
        let mut relAddr: i64 = 0;
        relAddr = i64::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.rel8.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, relAddr)];
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
        let mut block_0_len = 1u64 as u32;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let mut relAddr: i64 = 0;
        let rel8 = token_parser.TokenFieldrel8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rel8 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel8 {
    Var0(Rel8Var0),
}
impl TableRel8 {
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
            Rel8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc:641:1"]
#[derive(Clone, Debug)]
struct Rel16Var0 {
    rel16: TokenField_rel16,
}
impl Rel16Var0 {
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
        let mut relAddr: i64 = 0;
        relAddr = i64::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.rel16.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, relAddr)];
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
        let mut relAddr: i64 = 0;
        let rel16 = token_parser.TokenFieldrel16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rel16 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel16 {
    Var0(Rel16Var0),
}
impl TableRel16 {
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
            Rel16Var0::parse(tokens_param, &mut context_current, inst_start)
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
