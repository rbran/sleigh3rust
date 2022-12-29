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
        0 => Register::r0,
        1 => Register::r1,
        2 => Register::r2,
        3 => Register::sp,
        4 => Register::gp,
        5 => Register::tp,
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
        16 => Register::r16,
        17 => Register::r17,
        18 => Register::r18,
        19 => Register::r19,
        20 => Register::r20,
        21 => Register::r21,
        22 => Register::r22,
        23 => Register::r23,
        24 => Register::r24,
        25 => Register::r25,
        26 => Register::r26,
        27 => Register::r27,
        28 => Register::r28,
        29 => Register::r29,
        30 => Register::ep,
        31 => Register::lp,
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
        0 => Register::EIPC,
        1 => Register::EIPSW,
        2 => Register::FEPC,
        3 => Register::FEPSW,
        4 => Register::ECR,
        5 => Register::PSW,
        6 => Register::FPSR,
        7 => Register::FPEPC,
        8 => Register::FPST,
        9 => Register::FPCC,
        10 => Register::FPCFG,
        11 => Register::SCCFG,
        12 => Register::SCBP,
        13 => Register::EIIC,
        14 => Register::FEIC,
        15 => Register::DBIC,
        16 => Register::CTPC,
        17 => Register::CTPSW,
        18 => Register::DBPC,
        19 => Register::DBPSW,
        20 => Register::CTBP,
        21 => Register::DIR,
        22 => Register::DBG22,
        23 => Register::DBG23,
        24 => Register::DBG24,
        25 => Register::DBG25,
        26 => Register::DBG26,
        27 => Register::DBG27,
        28 => Register::EIWR,
        29 => Register::FEWR,
        30 => Register::DBWR,
        31 => Register::BSEL,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_2_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_2_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_2_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::r0r1,
        2 => Register::r2sp,
        4 => Register::r4r5,
        6 => Register::r6r7,
        8 => Register::r8r9,
        10 => Register::r10r11,
        12 => Register::r12r13,
        14 => Register::r14r15,
        16 => Register::r16r17,
        18 => Register::r18r19,
        20 => Register::r20r21,
        22 => Register::r22r23,
        24 => Register::r24r25,
        26 => Register::r26r27,
        28 => Register::r28r29,
        30 => Register::eplp,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_3_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_3_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_3_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::r0,
        1 => Register::r2,
        2 => Register::gp,
        3 => Register::r6,
        4 => Register::r8,
        5 => Register::r10,
        6 => Register::r12,
        7 => Register::r14,
        8 => Register::r16,
        9 => Register::r18,
        10 => Register::r20,
        11 => Register::r22,
        12 => Register::r24,
        13 => Register::r26,
        14 => Register::r28,
        15 => Register::ep,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_4_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_4_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_4_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::r1,
        1 => Register::sp,
        2 => Register::tp,
        3 => Register::r7,
        4 => Register::r9,
        5 => Register::r11,
        6 => Register::r13,
        7 => Register::r15,
        8 => Register::r17,
        9 => Register::r19,
        10 => Register::r21,
        11 => Register::r23,
        12 => Register::r25,
        13 => Register::r27,
        14 => Register::r29,
        15 => Register::lp,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_5_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => DisplayElement::Literal("f"),
        1 => DisplayElement::Literal("un"),
        2 => DisplayElement::Literal("eq"),
        3 => DisplayElement::Literal("ueq"),
        4 => DisplayElement::Literal("olt"),
        5 => DisplayElement::Literal("ult"),
        6 => DisplayElement::Literal("ole"),
        7 => DisplayElement::Literal("ule"),
        8 => DisplayElement::Literal("sd"),
        9 => DisplayElement::Literal("ngle"),
        10 => DisplayElement::Literal("seq"),
        11 => DisplayElement::Literal("ngl"),
        12 => DisplayElement::Literal("lt"),
        13 => DisplayElement::Literal("nge"),
        14 => DisplayElement::Literal("le"),
        15 => DisplayElement::Literal("ngt"),
        _ => unreachable!("Invalid Attach Value"),
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op0000(u8);
impl TokenField_op0000 {
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
struct TokenField_op0003(u8);
impl TokenField_op0003 {
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
struct TokenField_op0004(u8);
impl TokenField_op0004 {
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
struct TokenField__R0004(u8);
impl TokenField__R0004 {
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
struct TokenField_SR0004(u8);
impl TokenField_SR0004 {
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
struct TokenField_R0004x2(u8);
impl TokenField_R0004x2 {
    fn execution(&self) -> Register {
        meaning_2_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_2_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_s0004(i8);
impl TokenField_s0004 {
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
struct TokenField_s0005(i8);
impl TokenField_s0005 {
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
struct TokenField_op0005(u8);
impl TokenField_op0005 {
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
struct TokenField_op0006(u8);
impl TokenField_op0006 {
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
struct TokenField_op0010(u16);
impl TokenField_op0010 {
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
struct TokenField_op0015(u16);
impl TokenField_op0015 {
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
struct TokenField_op0106(u8);
impl TokenField_op0106 {
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
struct TokenField_op0406(u8);
impl TokenField_op0406 {
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
struct TokenField_op0410(u8);
impl TokenField_op0410 {
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
struct TokenField_op0505(u8);
impl TokenField_op0505 {
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
struct TokenField_op0510(u8);
impl TokenField_op0510 {
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
struct TokenField_op0515(u16);
impl TokenField_op0515 {
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
struct TokenField_op0610(u8);
impl TokenField_op0610 {
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
struct TokenField_op0615(u16);
impl TokenField_op0615 {
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
struct TokenField_op0710(u8);
impl TokenField_op0710 {
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
struct TokenField_op1113(u8);
impl TokenField_op1113 {
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
struct TokenField_op1114(u8);
impl TokenField_op1114 {
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
struct TokenField_op1115(u8);
impl TokenField_op1115 {
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
struct TokenField__R1115(u8);
impl TokenField__R1115 {
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
struct TokenField_SR1115(u8);
impl TokenField_SR1115 {
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
struct TokenField_R1115x2(u8);
impl TokenField_R1115x2 {
    fn execution(&self) -> Register {
        meaning_2_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_2_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_s1115(i8);
impl TokenField_s1115 {
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
struct TokenField_op1415(u8);
impl TokenField_op1415 {
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
struct TokenField_op1515(u8);
impl TokenField_op1515 {
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
struct TokenField_op1616(u8);
impl TokenField_op1616 {
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
struct TokenField_op1617(u8);
impl TokenField_op1617 {
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
struct TokenField_op1619(u8);
impl TokenField_op1619 {
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
struct TokenField_op1620(u8);
impl TokenField_op1620 {
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
struct TokenField_R1620(u8);
impl TokenField_R1620 {
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
struct TokenField_R1620x2(u8);
impl TokenField_R1620x2 {
    fn execution(&self) -> Register {
        meaning_2_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_2_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op1626(u16);
impl TokenField_op1626 {
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
struct TokenField_op1631(u16);
impl TokenField_op1631 {
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
struct TokenField_s1631(i16);
impl TokenField_s1631 {
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
struct TokenField_fcbit1719(u8);
impl TokenField_fcbit1719 {
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
struct TokenField_op1720(u8);
impl TokenField_op1720 {
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
struct TokenField_op0_1720(u8);
impl TokenField_op0_1720 {
    fn execution(&self) -> Register {
        meaning_3_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_3_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op1_1720(u8);
impl TokenField_op1_1720 {
    fn execution(&self) -> Register {
        meaning_4_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_4_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_s1731(i16);
impl TokenField_s1731 {
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
struct TokenField_op1821(u8);
impl TokenField_op1821 {
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
struct TokenField_s1821(i8);
impl TokenField_s1821 {
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
struct TokenField_op2020(u8);
impl TokenField_op2020 {
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
struct TokenField_op2026(u8);
impl TokenField_op2026 {
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
struct TokenField_op2122(u8);
impl TokenField_op2122 {
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
struct TokenField_op2126(u8);
impl TokenField_op2126 {
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
struct TokenField_op2226(u8);
impl TokenField_op2226 {
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
struct TokenField_op2323(u8);
impl TokenField_op2323 {
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
struct TokenField_op2426(u8);
impl TokenField_op2426 {
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
struct TokenField_op2729(u8);
impl TokenField_op2729 {
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
struct TokenField_fcond2730(u8);
impl TokenField_fcond2730 {
    fn execution(&self) -> u8 {
        self.0
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_5_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op2731(u8);
impl TokenField_op2731 {
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
struct TokenField__R2731(u8);
impl TokenField__R2731 {
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
struct TokenField_R2731x2(u8);
impl TokenField_R2731x2 {
    fn execution(&self) -> Register {
        meaning_2_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_2_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op3031(u8);
impl TokenField_op3031 {
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
struct TokenField_op3131(u8);
impl TokenField_op3131 {
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
struct TokenField_op3247(u16);
impl TokenField_op3247 {
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
struct TokenField_s3247(i16);
impl TokenField_s3247 {
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
struct TokenField_op4863(u16);
impl TokenField_op4863 {
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
struct TokenField_prep00(u8);
impl TokenField_prep00 {
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
struct TokenField_prep0105(u8);
impl TokenField_prep0105 {
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
struct TokenField_prep0615(u16);
impl TokenField_prep0615 {
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
struct TokenField_prep1620(u8);
impl TokenField_prep1620 {
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
struct TokenField_prep21(u8);
impl TokenField_prep21 {
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
struct TokenField_prep22(u8);
impl TokenField_prep22 {
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
struct TokenField_prep2223(u16);
impl TokenField_prep2223 {
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
struct TokenField_prep23(u8);
impl TokenField_prep23 {
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
struct TokenField_prep24(u8);
impl TokenField_prep24 {
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
struct TokenField_prep2431(u8);
impl TokenField_prep2431 {
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
struct TokenField_prep25(u8);
impl TokenField_prep25 {
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
struct TokenField_prep26(u8);
impl TokenField_prep26 {
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
struct TokenField_prep27(u8);
impl TokenField_prep27 {
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
struct TokenField_prep28(u8);
impl TokenField_prep28 {
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
struct TokenField_prep29(u8);
impl TokenField_prep29 {
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
struct TokenField_prep2931(u8);
impl TokenField_prep2931 {
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
struct TokenField_prep3031(u8);
impl TokenField_prep3031 {
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
struct TokenField_prep2427(u8);
impl TokenField_prep2427 {
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
struct TokenField_prep2527(u8);
impl TokenField_prep2527 {
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
struct TokenField_prep2627(u8);
impl TokenField_prep2627 {
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
struct TokenField_prep30(u8);
impl TokenField_prep30 {
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
struct TokenField_prep31(u8);
impl TokenField_prep31 {
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
struct TokenParser<const LEN: usize>([u8; LEN]);
impl<const LEN: usize> TokenParser<LEN> {
    fn new(data: &[u8]) -> Option<Self> {
        let token_slice: &[u8] = data.get(..LEN)?;
        let token_data = <[u8; LEN]>::try_from(token_slice).unwrap();
        Some(Self(token_data))
    }
    fn op0000(&self) -> TokenField_op0000 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0000(inner_value)
    }
    fn op0003(&self) -> TokenField_op0003 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0003(inner_value)
    }
    fn op0004(&self) -> TokenField_op0004 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0004(inner_value)
    }
    fn _R0004(&self) -> TokenField__R0004 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField__R0004(inner_value)
    }
    fn SR0004(&self) -> TokenField_SR0004 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_SR0004(inner_value)
    }
    fn R0004x2(&self) -> TokenField_R0004x2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_R0004x2(inner_value)
    }
    fn s0004(&self) -> TokenField_s0004 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_s0004(inner_value)
    }
    fn s0005(&self) -> TokenField_s0005 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<false>(work_value, 0u64 as usize, 6u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_s0005(inner_value)
    }
    fn op0005(&self) -> TokenField_op0005 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0005(inner_value)
    }
    fn op0006(&self) -> TokenField_op0006 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0006(inner_value)
    }
    fn op0010(&self) -> TokenField_op0010 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 0u64 as usize, 11u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op0010(inner_value)
    }
    fn op0015(&self) -> TokenField_op0015 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op0015(inner_value)
    }
    fn op0106(&self) -> TokenField_op0106 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0106(inner_value)
    }
    fn op0406(&self) -> TokenField_op0406 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 4u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0406(inner_value)
    }
    fn op0410(&self) -> TokenField_op0410 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 4u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0410(inner_value)
    }
    fn op0505(&self) -> TokenField_op0505 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 5u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0505(inner_value)
    }
    fn op0510(&self) -> TokenField_op0510 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 5u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0510(inner_value)
    }
    fn op0515(&self) -> TokenField_op0515 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 5u64 as usize, 11u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op0515(inner_value)
    }
    fn op0610(&self) -> TokenField_op0610 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 6u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0610(inner_value)
    }
    fn op0615(&self) -> TokenField_op0615 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 6u64 as usize, 10u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op0615(inner_value)
    }
    fn op0710(&self) -> TokenField_op0710 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 7u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0710(inner_value)
    }
    fn op1113(&self) -> TokenField_op1113 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1113(inner_value)
    }
    fn op1114(&self) -> TokenField_op1114 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1114(inner_value)
    }
    fn op1115(&self) -> TokenField_op1115 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1115(inner_value)
    }
    fn _R1115(&self) -> TokenField__R1115 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField__R1115(inner_value)
    }
    fn SR1115(&self) -> TokenField_SR1115 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_SR1115(inner_value)
    }
    fn R1115x2(&self) -> TokenField_R1115x2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_R1115x2(inner_value)
    }
    fn s1115(&self) -> TokenField_s1115 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_s1115(inner_value)
    }
    fn op1415(&self) -> TokenField_op1415 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 6u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1415(inner_value)
    }
    fn op1515(&self) -> TokenField_op1515 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1515(inner_value)
    }
    fn op1616(&self) -> TokenField_op1616 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1616(inner_value)
    }
    fn op1617(&self) -> TokenField_op1617 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1617(inner_value)
    }
    fn op1619(&self) -> TokenField_op1619 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1619(inner_value)
    }
    fn op1620(&self) -> TokenField_op1620 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1620(inner_value)
    }
    fn R1620(&self) -> TokenField_R1620 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_R1620(inner_value)
    }
    fn R1620x2(&self) -> TokenField_R1620x2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_R1620x2(inner_value)
    }
    fn op1626(&self) -> TokenField_op1626 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 0u64 as usize, 11u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op1626(inner_value)
    }
    fn op1631(&self) -> TokenField_op1631 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op1631(inner_value)
    }
    fn s1631(&self) -> TokenField_s1631 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            i16::try_from(value).unwrap()
        };
        TokenField_s1631(inner_value)
    }
    fn fcbit1719(&self) -> TokenField_fcbit1719 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_fcbit1719(inner_value)
    }
    fn op1720(&self) -> TokenField_op1720 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1720(inner_value)
    }
    fn op0_1720(&self) -> TokenField_op0_1720 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op0_1720(inner_value)
    }
    fn op1_1720(&self) -> TokenField_op1_1720 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1_1720(inner_value)
    }
    fn s1731(&self) -> TokenField_s1731 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i16::<false>(work_value, 1u64 as usize, 15u64 as usize);
            i16::try_from(value).unwrap()
        };
        TokenField_s1731(inner_value)
    }
    fn op1821(&self) -> TokenField_op1821 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 2u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op1821(inner_value)
    }
    fn s1821(&self) -> TokenField_s1821 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i8::<false>(work_value, 2u64 as usize, 4u64 as usize);
            i8::try_from(value).unwrap()
        };
        TokenField_s1821(inner_value)
    }
    fn op2020(&self) -> TokenField_op2020 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 4u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2020(inner_value)
    }
    fn op2026(&self) -> TokenField_op2026 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 4u64 as usize, 7u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2026(inner_value)
    }
    fn op2122(&self) -> TokenField_op2122 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 5u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2122(inner_value)
    }
    fn op2126(&self) -> TokenField_op2126 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 5u64 as usize, 6u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2126(inner_value)
    }
    fn op2226(&self) -> TokenField_op2226 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 6u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2226(inner_value)
    }
    fn op2323(&self) -> TokenField_op2323 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2323(inner_value)
    }
    fn op2426(&self) -> TokenField_op2426 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2426(inner_value)
    }
    fn op2729(&self) -> TokenField_op2729 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2729(inner_value)
    }
    fn fcond2730(&self) -> TokenField_fcond2730 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_fcond2730(inner_value)
    }
    fn op2731(&self) -> TokenField_op2731 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op2731(inner_value)
    }
    fn _R2731(&self) -> TokenField__R2731 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField__R2731(inner_value)
    }
    fn R2731x2(&self) -> TokenField_R2731x2 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_R2731x2(inner_value)
    }
    fn op3031(&self) -> TokenField_op3031 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 6u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op3031(inner_value)
    }
    fn op3131(&self) -> TokenField_op3131 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 1u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_op3131(inner_value)
    }
    fn op3247(&self) -> TokenField_op3247 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op3247(inner_value)
    }
    fn s3247(&self) -> TokenField_s3247 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_i16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            i16::try_from(value).unwrap()
        };
        TokenField_s3247(inner_value)
    }
    fn op4863(&self) -> TokenField_op4863 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 0u64 as usize, 16u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_op4863(inner_value)
    }
    fn prep00(&self) -> TokenField_prep00 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep00(inner_value)
    }
    fn prep0105(&self) -> TokenField_prep0105 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 1u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep0105(inner_value)
    }
    fn prep0615(&self) -> TokenField_prep0615 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 0u64 as usize;
            let token_end = 2u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 6u64 as usize, 10u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_prep0615(inner_value)
    }
    fn prep1620(&self) -> TokenField_prep1620 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 2u64 as usize;
            let token_end = 3u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 5u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep1620(inner_value)
    }
    fn prep21(&self) -> TokenField_prep21 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 2u64 as usize;
            let token_end = 3u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 5u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep21(inner_value)
    }
    fn prep22(&self) -> TokenField_prep22 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 2u64 as usize;
            let token_end = 3u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 6u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep22(inner_value)
    }
    fn prep2223(&self) -> TokenField_prep2223 {
        let inner_value = {
            let mut work_value = [0u8; 2u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 2u64 as usize;
            let token_start = 2u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u16::<false>(work_value, 6u64 as usize, 10u64 as usize);
            u16::try_from(value).unwrap()
        };
        TokenField_prep2223(inner_value)
    }
    fn prep23(&self) -> TokenField_prep23 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 2u64 as usize;
            let token_end = 3u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep23(inner_value)
    }
    fn prep24(&self) -> TokenField_prep24 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep24(inner_value)
    }
    fn prep2431(&self) -> TokenField_prep2431 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 8u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep2431(inner_value)
    }
    fn prep25(&self) -> TokenField_prep25 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep25(inner_value)
    }
    fn prep26(&self) -> TokenField_prep26 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 2u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep26(inner_value)
    }
    fn prep27(&self) -> TokenField_prep27 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 3u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep27(inner_value)
    }
    fn prep28(&self) -> TokenField_prep28 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 4u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep28(inner_value)
    }
    fn prep29(&self) -> TokenField_prep29 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 5u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep29(inner_value)
    }
    fn prep2931(&self) -> TokenField_prep2931 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 5u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep2931(inner_value)
    }
    fn prep3031(&self) -> TokenField_prep3031 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 6u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep3031(inner_value)
    }
    fn prep2427(&self) -> TokenField_prep2427 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 0u64 as usize, 4u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep2427(inner_value)
    }
    fn prep2527(&self) -> TokenField_prep2527 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 1u64 as usize, 3u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep2527(inner_value)
    }
    fn prep2627(&self) -> TokenField_prep2627 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 2u64 as usize, 2u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep2627(inner_value)
    }
    fn prep30(&self) -> TokenField_prep30 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 6u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep30(inner_value)
    }
    fn prep31(&self) -> TokenField_prep31 {
        let inner_value = {
            let mut work_value = [0u8; 1u64 as usize];
            let work_start = 0u64 as usize;
            let work_end = 1u64 as usize;
            let token_start = 3u64 as usize;
            let token_end = 4u64 as usize;
            work_value[work_start..work_end]
                .copy_from_slice(&self.0[token_start..token_end]);
            let value =
                read_u8::<false>(work_value, 7u64 as usize, 1u64 as usize);
            u8::try_from(value).unwrap()
        };
        TokenField_prep31(inner_value)
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    r0,
    r1,
    r2,
    sp,
    gp,
    tp,
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
    r16,
    r17,
    r18,
    r19,
    r20,
    r21,
    r22,
    r23,
    r24,
    r25,
    r26,
    r27,
    r28,
    r29,
    ep,
    lp,
    EIPC,
    EIPSW,
    FEPC,
    FEPSW,
    ECR,
    PSW,
    FPSR,
    FPEPC,
    FPST,
    FPCC,
    FPCFG,
    SCCFG,
    SCBP,
    EIIC,
    FEIC,
    DBIC,
    CTPC,
    CTPSW,
    DBPC,
    DBPSW,
    CTBP,
    DIR,
    DBG22,
    DBG23,
    DBG24,
    DBG25,
    DBG26,
    DBG27,
    EIWR,
    FEWR,
    DBWR,
    BSEL,
    r0r1,
    r2sp,
    r4r5,
    r6r7,
    r8r9,
    r10r11,
    r12r13,
    r14r15,
    r16r17,
    r18r19,
    r20r21,
    r22r23,
    r24r25,
    r26r27,
    r28r29,
    eplp,
    PC,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::r0 => write!(f, "r0"),
            Self::r1 => write!(f, "r1"),
            Self::r2 => write!(f, "r2"),
            Self::sp => write!(f, "sp"),
            Self::gp => write!(f, "gp"),
            Self::tp => write!(f, "tp"),
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
            Self::r16 => write!(f, "r16"),
            Self::r17 => write!(f, "r17"),
            Self::r18 => write!(f, "r18"),
            Self::r19 => write!(f, "r19"),
            Self::r20 => write!(f, "r20"),
            Self::r21 => write!(f, "r21"),
            Self::r22 => write!(f, "r22"),
            Self::r23 => write!(f, "r23"),
            Self::r24 => write!(f, "r24"),
            Self::r25 => write!(f, "r25"),
            Self::r26 => write!(f, "r26"),
            Self::r27 => write!(f, "r27"),
            Self::r28 => write!(f, "r28"),
            Self::r29 => write!(f, "r29"),
            Self::ep => write!(f, "ep"),
            Self::lp => write!(f, "lp"),
            Self::EIPC => write!(f, "EIPC"),
            Self::EIPSW => write!(f, "EIPSW"),
            Self::FEPC => write!(f, "FEPC"),
            Self::FEPSW => write!(f, "FEPSW"),
            Self::ECR => write!(f, "ECR"),
            Self::PSW => write!(f, "PSW"),
            Self::FPSR => write!(f, "FPSR"),
            Self::FPEPC => write!(f, "FPEPC"),
            Self::FPST => write!(f, "FPST"),
            Self::FPCC => write!(f, "FPCC"),
            Self::FPCFG => write!(f, "FPCFG"),
            Self::SCCFG => write!(f, "SCCFG"),
            Self::SCBP => write!(f, "SCBP"),
            Self::EIIC => write!(f, "EIIC"),
            Self::FEIC => write!(f, "FEIC"),
            Self::DBIC => write!(f, "DBIC"),
            Self::CTPC => write!(f, "CTPC"),
            Self::CTPSW => write!(f, "CTPSW"),
            Self::DBPC => write!(f, "DBPC"),
            Self::DBPSW => write!(f, "DBPSW"),
            Self::CTBP => write!(f, "CTBP"),
            Self::DIR => write!(f, "DIR"),
            Self::DBG22 => write!(f, "DBG22"),
            Self::DBG23 => write!(f, "DBG23"),
            Self::DBG24 => write!(f, "DBG24"),
            Self::DBG25 => write!(f, "DBG25"),
            Self::DBG26 => write!(f, "DBG26"),
            Self::DBG27 => write!(f, "DBG27"),
            Self::EIWR => write!(f, "EIWR"),
            Self::FEWR => write!(f, "FEWR"),
            Self::DBWR => write!(f, "DBWR"),
            Self::BSEL => write!(f, "BSEL"),
            Self::r0r1 => write!(f, "r0r1"),
            Self::r2sp => write!(f, "r2sp"),
            Self::r4r5 => write!(f, "r4r5"),
            Self::r6r7 => write!(f, "r6r7"),
            Self::r8r9 => write!(f, "r8r9"),
            Self::r10r11 => write!(f, "r10r11"),
            Self::r12r13 => write!(f, "r12r13"),
            Self::r14r15 => write!(f, "r14r15"),
            Self::r16r17 => write!(f, "r16r17"),
            Self::r18r19 => write!(f, "r18r19"),
            Self::r20r21 => write!(f, "r20r21"),
            Self::r22r23 => write!(f, "r22r23"),
            Self::r24r25 => write!(f, "r24r25"),
            Self::r26r27 => write!(f, "r26r27"),
            Self::r28r29 => write!(f, "r28r29"),
            Self::eplp => write!(f, "eplp"),
            Self::PC => write!(f, "PC"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:142:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("di")];
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
        if token_parser.op0015().disassembly() != 2016 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 352 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:166:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ei")];
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
        if token_parser.op0015().disassembly() != 34784 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 352 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:204:1"]
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
            [DisplayElement::Literal("halt")];
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
        if token_parser.op0015().disassembly() != 2016 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 288 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:7:1"]
#[derive(Clone, Debug)]
struct instructionVar3 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("absf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 0 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 24 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:13:1"]
#[derive(Clone, Debug)]
struct instructionVar4 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("absf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 8 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:31:1"]
#[derive(Clone, Debug)]
struct instructionVar5 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ceilf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 2 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:38:1"]
#[derive(Clone, Debug)]
struct instructionVar6 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ceilf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 18 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:45:1"]
#[derive(Clone, Debug)]
struct instructionVar7 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ceilf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 18 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:51:1"]
#[derive(Clone, Debug)]
struct instructionVar8 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ceilf.dw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 2 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:57:1"]
#[derive(Clone, Debug)]
struct instructionVar9 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
            DisplayElement::Literal("ceilf.sl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 2 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:64:1"]
#[derive(Clone, Debug)]
struct instructionVar10 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
            DisplayElement::Literal("ceilf.sul"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 18 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:71:1"]
#[derive(Clone, Debug)]
struct instructionVar11 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ceilf.sul"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 18 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:77:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ceilf.sw"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 2 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:126:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("cvtf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 4 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:132:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cvtf.ds"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 3 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 18 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:138:1"]
#[derive(Clone, Debug)]
struct instructionVar15 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("cvtf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 20 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:144:1"]
#[derive(Clone, Debug)]
struct instructionVar16 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cvtf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 20 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:150:1"]
#[derive(Clone, Debug)]
struct instructionVar17 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cvtf.sw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 4 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:156:1"]
#[derive(Clone, Debug)]
struct instructionVar18 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("cvtf.ls"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 18 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:162:1"]
#[derive(Clone, Debug)]
struct instructionVar19 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cvtf.ls"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 2 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:168:1"]
#[derive(Clone, Debug)]
struct instructionVar20 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("cvtf.sd"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 2 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 18 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:174:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("cvtf.sl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 4 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:180:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("cvtf.sul"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 20 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:186:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("cvtf.suw"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 20 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:192:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("cvtf.sw"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 4 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:198:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("cvtf.uls"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 17 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 18 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:204:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cvtf.uls"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 17 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 2 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:210:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("cvtf.uwd"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 16 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 18 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:216:1"]
#[derive(Clone, Debug)]
struct instructionVar28 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("cvtf.uws"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 16 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 2 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:222:1"]
#[derive(Clone, Debug)]
struct instructionVar29 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
            DisplayElement::Literal("cvtf.wd"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 18 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:228:1"]
#[derive(Clone, Debug)]
struct instructionVar30 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("cvtf.ws"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 2 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:246:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("floorf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 3 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:253:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("floorf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 19 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:260:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("floorf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 19 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:266:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("floorf.dw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 3 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:272:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("floorf.sl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 3 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:279:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
            DisplayElement::Literal("floorf.sul"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 19 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:286:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("floorf.suw"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 19 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:292:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("floorf.suw"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 3 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:350:1"]
#[derive(Clone, Debug)]
struct instructionVar39 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("negf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 24 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:356:1"]
#[derive(Clone, Debug)]
struct instructionVar40 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("negf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 8 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:374:1"]
#[derive(Clone, Debug)]
struct instructionVar41 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("recipf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 30 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:380:1"]
#[derive(Clone, Debug)]
struct instructionVar42 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("recipf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 14 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:386:1"]
#[derive(Clone, Debug)]
struct instructionVar43 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("rsqrtf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 2 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 30 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:392:1"]
#[derive(Clone, Debug)]
struct instructionVar44 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("rsqrtf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 2 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 14 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:398:1"]
#[derive(Clone, Debug)]
struct instructionVar45 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("sqrtf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 0 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 30 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:404:1"]
#[derive(Clone, Debug)]
struct instructionVar46 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("sqrtf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 14 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:429:1"]
#[derive(Clone, Debug)]
struct instructionVar47 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("trncf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:435:1"]
#[derive(Clone, Debug)]
struct instructionVar48 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
            DisplayElement::Literal("trncf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 17 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:441:1"]
#[derive(Clone, Debug)]
struct instructionVar49 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("trncf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 17 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:447:1"]
#[derive(Clone, Debug)]
struct instructionVar50 {
    R1115x2: TokenField_R1115x2,
    R2731: R2731,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("trncf.dw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:453:1"]
#[derive(Clone, Debug)]
struct instructionVar51 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
            DisplayElement::Literal("trncf.sl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:459:1"]
#[derive(Clone, Debug)]
struct instructionVar52 {
    R2731x2: TokenField_R2731x2,
    R1115: R1115,
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
            DisplayElement::Literal("trncf.sul"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.R2731x2.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 17 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:465:1"]
#[derive(Clone, Debug)]
struct instructionVar53 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("trncf.suw"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 17 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:471:1"]
#[derive(Clone, Debug)]
struct instructionVar54 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("trncf.sw"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 1 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 34 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:221:1"]
#[derive(Clone, Debug)]
struct instructionVar55 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bsh"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 834 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:235:1"]
#[derive(Clone, Debug)]
struct instructionVar56 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bsw"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 832 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:261:1"]
#[derive(Clone, Debug)]
struct instructionVar57 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("hsh"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 838 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:271:1"]
#[derive(Clone, Debug)]
struct instructionVar58 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("hsw"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 836 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:299:1"]
#[derive(Clone, Debug)]
struct instructionVar59 {
    c0003: c0003,
    R1115: R1115,
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("sasf")];
        display.extend_from_slice(&extend);
        self.c0003.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0410().disassembly() != 126 {
            return None;
        }
        let c0003 = if let Some((len, table)) =
            c0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 512 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { c0003, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:305:1"]
#[derive(Clone, Debug)]
struct instructionVar60 {
    c0003: c0003,
    R1115: R1115,
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("setf")];
        display.extend_from_slice(&extend);
        self.c0003.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0410().disassembly() != 126 {
            return None;
        }
        let c0003 = if let Some((len, table)) =
            c0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 0 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { c0003, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:133:1"]
#[derive(Clone, Debug)]
struct instructionVar61 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ctret")];
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
        if token_parser.op0515().disassembly() != 63 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 324 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:173:1"]
#[derive(Clone, Debug)]
struct instructionVar62 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("eiret")];
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
        if token_parser.op0515().disassembly() != 63 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 328 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:181:1"]
#[derive(Clone, Debug)]
struct instructionVar63 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("feret")];
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
        if token_parser.op0515().disassembly() != 63 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 330 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:262:1"]
#[derive(Clone, Debug)]
struct instructionVar64 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("reti")];
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
        if token_parser.op0515().disassembly() != 63 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 320 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:300:1"]
#[derive(Clone, Debug)]
struct instructionVar65 {
    op1115: TokenField_op1115,
    op0003: TokenField_op0003,
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
            DisplayElement::Literal("rie"),
            DisplayElement::Literal(" "),
            self.op1115.display(),
            DisplayElement::Literal(", "),
            self.op0003.display(),
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
        if token_parser.op0410().disassembly() != 127 {
            return None;
        }
        let op1115 = token_parser.op1115();
        let op0003 = token_parser.op0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 0 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1115, op0003 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:361:1"]
#[derive(Clone, Debug)]
struct instructionVar66 {
    op0004: TokenField_op0004,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("trap"),
            DisplayElement::Literal(" "),
            self.op0004.display(),
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
        if token_parser.op0515().disassembly() != 63 {
            return None;
        }
        let op0004 = token_parser.op0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 256 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:281:1"]
#[derive(Clone, Debug)]
struct instructionVar67 {
    R0004: R0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sar"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 160 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:311:1"]
#[derive(Clone, Debug)]
struct instructionVar68 {
    R0004: R0004,
    R1115: R1115,
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
            [DisplayElement::Literal("shl"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 192 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:329:1"]
#[derive(Clone, Debug)]
struct instructionVar69 {
    R0004: R0004,
    R1115: R1115,
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
            [DisplayElement::Literal("shr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:113:1"]
#[derive(Clone, Debug)]
struct instructionVar70 {
    R0004: R0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("clr1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", [")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 228 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:130:1"]
#[derive(Clone, Debug)]
struct instructionVar71 {
    R0004: R0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("not1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", [")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 226 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:147:1"]
#[derive(Clone, Debug)]
struct instructionVar72 {
    R0004: R0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("set1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", [")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 224 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:163:1"]
#[derive(Clone, Debug)]
struct instructionVar73 {
    R0004: R0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("tst1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", [")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 230 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:13:1"]
#[derive(Clone, Debug)]
struct instructionVar74 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sch0l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 868 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:21:1"]
#[derive(Clone, Debug)]
struct instructionVar75 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("sch0r"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 864 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:29:1"]
#[derive(Clone, Debug)]
struct instructionVar76 {
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("sch1l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 870 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:37:1"]
#[derive(Clone, Debug)]
struct instructionVar77 {
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sch1r"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0010().disassembly() != 2016 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 866 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:210:1"]
#[derive(Clone, Debug)]
struct instructionVar78 {
    SR1115: TokenField_SR1115,
    R0004: R0004,
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
            DisplayElement::Literal("ldsr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.SR1115.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let SR1115 = token_parser.SR1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 32 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, SR1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:312:1"]
#[derive(Clone, Debug)]
struct instructionVar79 {
    SR0004: TokenField_SR0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("stsr"),
            DisplayElement::Literal(" "),
            self.SR0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let SR0004 = token_parser.SR0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1631().disassembly() != 64 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, SR0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:345:1"]
#[derive(Clone, Debug)]
struct instructionVar80 {
    op2729: TokenField_op2729,
    op0004: TokenField_op0004,
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
        let mut vector8: i64 = 0;
        vector8 = ((self.op2729.disassembly() << (5u64 as i64))
            | self.op0004.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("syscall"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, vector8),
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
        if token_parser.op0515().disassembly() != 1727 {
            return None;
        }
        let mut vector8: i64 = 0;
        vector8 = ((token_parser.op2729().disassembly() << (5u64 as i64))
            | token_parser.op0004().disassembly());
        let op0004 = token_parser.op0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op3031().disassembly() != 0 {
            return None;
        }
        if token_parser.op1626().disassembly() != 352 {
            return None;
        }
        let op2729 = token_parser.op2729();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0004, op2729 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:16:1"]
#[derive(Clone, Debug)]
struct instructionVar81 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
            [DisplayElement::Literal("mul"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 544 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:51:1"]
#[derive(Clone, Debug)]
struct instructionVar82 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mulu"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 546 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:199:1"]
#[derive(Clone, Debug)]
struct instructionVar83 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("satadd"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 954 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:217:1"]
#[derive(Clone, Debug)]
struct instructionVar84 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
            DisplayElement::Literal("satsub"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 922 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:251:1"]
#[derive(Clone, Debug)]
struct instructionVar85 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
            [DisplayElement::Literal("div"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 704 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:272:1"]
#[derive(Clone, Debug)]
struct instructionVar86 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("divh"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 640 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:284:1"]
#[derive(Clone, Debug)]
struct instructionVar87 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("divhu"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 642 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:296:1"]
#[derive(Clone, Debug)]
struct instructionVar88 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("divu"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 706 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:315:1"]
#[derive(Clone, Debug)]
struct instructionVar89 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("divq"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 764 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:327:1"]
#[derive(Clone, Debug)]
struct instructionVar90 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("divqu"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 766 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:19:1"]
#[derive(Clone, Debug)]
struct instructionVar91 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("addf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        let R0004x2 = token_parser.R0004x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 16 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115x2,
                R0004x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:25:1"]
#[derive(Clone, Debug)]
struct instructionVar92 {
    R1115: R1115,
    R0004: R0004,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("addf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:234:1"]
#[derive(Clone, Debug)]
struct instructionVar93 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("divf.s"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004x2 = token_parser.R0004x2();
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 30 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:240:1"]
#[derive(Clone, Debug)]
struct instructionVar94 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("divf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 14 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:304:1"]
#[derive(Clone, Debug)]
struct instructionVar95 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("maxf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004x2 = token_parser.R0004x2();
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 24 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:311:1"]
#[derive(Clone, Debug)]
struct instructionVar96 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("maxf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 8 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:318:1"]
#[derive(Clone, Debug)]
struct instructionVar97 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("minf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004x2 = token_parser.R0004x2();
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 26 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:325:1"]
#[derive(Clone, Debug)]
struct instructionVar98 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("minf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 10 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:338:1"]
#[derive(Clone, Debug)]
struct instructionVar99 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mulf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004x2 = token_parser.R0004x2();
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 20 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:344:1"]
#[derive(Clone, Debug)]
struct instructionVar100 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mulf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 4 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:410:1"]
#[derive(Clone, Debug)]
struct instructionVar101 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("subf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004x2 = token_parser.R0004x2();
        let R1115x2 = token_parser.R1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 18 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:416:1"]
#[derive(Clone, Debug)]
struct instructionVar102 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("subf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 35 {
            return None;
        }
        if token_parser.op1620().disassembly() != 2 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:293:1"]
#[derive(Clone, Debug)]
struct instructionVar103 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sar"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 162 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:323:1"]
#[derive(Clone, Debug)]
struct instructionVar104 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("shl"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 194 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:341:1"]
#[derive(Clone, Debug)]
struct instructionVar105 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("shr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 130 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:123:1"]
#[derive(Clone, Debug)]
struct instructionVar106 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("caxi"),
            DisplayElement::Literal(" ["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1626().disassembly() != 238 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:24:1"]
#[derive(Clone, Debug)]
struct instructionVar107 {
    s1821: TokenField_s1821,
    op0004: TokenField_op0004,
    R1115: R1115,
    R2731: R2731,
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
        let mut imm9: i64 = 0;
        imm9 = ((self.s1821.disassembly() << (5u64 as i64))
            | self.op0004.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mul"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, imm9),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let mut imm9: i64 = 0;
        imm9 = ((token_parser.s1821().disassembly() << (5u64 as i64))
            | token_parser.op0004().disassembly());
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.op0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2226().disassembly() != 9 {
            return None;
        }
        if token_parser.op1617().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s1821 = token_parser.s1821();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R2731,
                op0004,
                s1821,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:77:1"]
#[derive(Clone, Debug)]
struct instructionVar108 {
    R0004: R0004,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("jmp"),
            DisplayElement::Literal(" ["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0515().disassembly() != 3 {
            return None;
        }
        if token_parser.op0004().disassembly() != 31 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:81:1"]
#[derive(Clone, Debug)]
struct instructionVar109 {
    R0004: R0004,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("jmp"),
            DisplayElement::Literal(" ["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0515().disassembly() != 3 {
            return None;
        }
        if token_parser.op0004().disassembly() == 31 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:100:1"]
#[derive(Clone, Debug)]
struct instructionVar110 {
    adr32: adr32,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("jr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr32.display_extend(
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
        if token_parser.op0015().disassembly() != 736 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let adr32 = if let Some((len, table)) =
            adr32::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { adr32 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:189:1"]
#[derive(Clone, Debug)]
struct instructionVar111 {
    op1114: TokenField_op1114,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fetrap"),
            DisplayElement::Literal(" "),
            self.op1114.display(),
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
        if token_parser.op0010().disassembly() != 64 {
            return None;
        }
        if token_parser.op1515().disassembly() != 0 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let op1114 = token_parser.op1114();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1114 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:216:1"]
#[derive(Clone, Debug)]
struct instructionVar112 {}
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
        if token_parser.op0015().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:288:1"]
#[derive(Clone, Debug)]
struct instructionVar113 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("rie")];
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
        if token_parser.op0015().disassembly() != 64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:327:1"]
#[derive(Clone, Debug)]
struct instructionVar114 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("synce")];
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
        if token_parser.op0015().disassembly() != 29 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:333:1"]
#[derive(Clone, Debug)]
struct instructionVar115 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("syncm")];
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
        if token_parser.op0015().disassembly() != 30 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:339:1"]
#[derive(Clone, Debug)]
struct instructionVar116 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("syncp")];
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
        if token_parser.op0015().disassembly() != 31 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:33:1"]
#[derive(Clone, Debug)]
struct instructionVar117 {
    R0004: R0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mulh"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 7 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:133:1"]
#[derive(Clone, Debug)]
struct instructionVar118 {
    s0004: TokenField_s0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mov"),
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 16 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.s0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:181:1"]
#[derive(Clone, Debug)]
struct instructionVar119 {
    R0004: R0004,
    R1115: R1115,
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
            DisplayElement::Literal("satadd"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 6 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:190:1"]
#[derive(Clone, Debug)]
struct instructionVar120 {
    s0004: TokenField_s0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("satadd"),
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 17 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.s0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:208:1"]
#[derive(Clone, Debug)]
struct instructionVar121 {
    R0004: R0004,
    R1115: R1115,
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
            DisplayElement::Literal("satsub"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 5 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:235:1"]
#[derive(Clone, Debug)]
struct instructionVar122 {
    R0004: R0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("satsubr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 4 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:98:1"]
#[derive(Clone, Debug)]
struct instructionVar123 {
    op0003: TokenField_op0003,
    R1115: R1115,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("sld.bu"),
            DisplayElement::Literal(" "),
            self.op0003.display(),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("], "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0410().disassembly() != 6 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0003 = token_parser.op0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0003 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:113:1"]
#[derive(Clone, Debug)]
struct instructionVar124 {
    op0003: TokenField_op0003,
    R1115: R1115,
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
        let mut disp5: i64 = 0;
        disp5 = (self.op0003.disassembly() * (2u64 as i64));
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("sld.hu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp5),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("], "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0410().disassembly() != 7 {
            return None;
        }
        let mut disp5: i64 = 0;
        disp5 = (token_parser.op0003().disassembly() * (2u64 as i64));
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0003 = token_parser.op0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0003 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:347:1"]
#[derive(Clone, Debug)]
struct instructionVar125 {
    R0004: R0004,
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
            [DisplayElement::Literal("sxb"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
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
        if token_parser.op0515().disassembly() != 5 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:353:1"]
#[derive(Clone, Debug)]
struct instructionVar126 {
    R0004: R0004,
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
            [DisplayElement::Literal("sxh"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
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
        if token_parser.op0515().disassembly() != 7 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:359:1"]
#[derive(Clone, Debug)]
struct instructionVar127 {
    R0004: R0004,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("zxb"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
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
        if token_parser.op0515().disassembly() != 4 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:365:1"]
#[derive(Clone, Debug)]
struct instructionVar128 {
    R0004: R0004,
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
            [DisplayElement::Literal("zxh"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
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
        if token_parser.op0515().disassembly() != 6 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:70:1"]
#[derive(Clone, Debug)]
struct instructionVar129 {
    R0004: R0004,
    adr32: adr32,
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
            DisplayElement::Literal("jarl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.adr32.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
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
        if token_parser.op0515().disassembly() != 23 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let adr32 = if let Some((len, table)) =
            adr32::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, adr32 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:87:1"]
#[derive(Clone, Debug)]
struct instructionVar130 {
    R0004: R0004,
    adr32i: adr32i,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("jmp"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr32i.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("[")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0515().disassembly() != 55 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let adr32i = if let Some((len, table)) =
            adr32i::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, adr32i }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:318:1"]
#[derive(Clone, Debug)]
struct instructionVar131 {
    R0004: R0004,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("switch"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
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
        if token_parser.op0515().disassembly() != 2 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:39:1"]
#[derive(Clone, Debug)]
struct instructionVar132 {
    s0004: TokenField_s0004,
    R1115: R1115,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mulh"),
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 23 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.s0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:59:1"]
#[derive(Clone, Debug)]
struct instructionVar133 {
    op1821: TokenField_op1821,
    op0004: TokenField_op0004,
    R1115: R1115,
    R2731: R2731,
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
        let mut imm9: i64 = 0;
        imm9 = ((self.op1821.disassembly() << (5u64 as i64))
            | self.op0004.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mulu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, imm9),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let mut imm9: i64 = 0;
        imm9 = ((token_parser.op1821().disassembly() << (5u64 as i64))
            | token_parser.op0004().disassembly());
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.op0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2226().disassembly() != 9 {
            return None;
        }
        if token_parser.op1617().disassembly() != 2 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1821 = token_parser.op1821();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R2731,
                op0004,
                op1821,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:422:1"]
#[derive(Clone, Debug)]
struct instructionVar134 {
    fcbit1719: TokenField_fcbit1719,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("trfsr"),
            DisplayElement::Literal(" "),
            self.fcbit1719.display(),
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
        if token_parser.op1115().disassembly() != 0 {
            return None;
        }
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        if token_parser.op0004().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2731().disassembly() != 0 {
            return None;
        }
        if token_parser.op2126().disassembly() != 32 {
            return None;
        }
        if token_parser.op2020().disassembly() != 0 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let fcbit1719 = token_parser.fcbit1719();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fcbit1719 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:98:1"]
#[derive(Clone, Debug)]
struct instructionVar135 {
    fcond2730: TokenField_fcond2730,
    R1115x2: TokenField_R1115x2,
    R0004x2: TokenField_R0004x2,
    fcbit1719: TokenField_fcbit1719,
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
        let extend: [DisplayElement; 9usize] = [
            DisplayElement::Literal("cmpf.d"),
            DisplayElement::Literal(" "),
            self.fcond2730.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.fcbit1719.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        let R0004x2 = token_parser.R0004x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op3131().disassembly() != 0 {
            return None;
        }
        if token_parser.op2126().disassembly() != 33 {
            return None;
        }
        if token_parser.op2020().disassembly() != 1 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let fcond2730 = token_parser.fcond2730();
        let fcbit1719 = token_parser.fcbit1719();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115x2,
                R0004x2,
                fcond2730,
                fcbit1719,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:115:1"]
#[derive(Clone, Debug)]
struct instructionVar136 {
    fcond2730: TokenField_fcond2730,
    fcbit1719: TokenField_fcbit1719,
    R1115: R1115,
    R0004: R0004,
}
impl instructionVar136 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmpf.s"),
            DisplayElement::Literal(" "),
            self.fcond2730.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.fcbit1719.display()];
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op3131().disassembly() != 0 {
            return None;
        }
        if token_parser.op2126().disassembly() != 33 {
            return None;
        }
        if token_parser.op2020().disassembly() != 0 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let fcond2730 = token_parser.fcond2730();
        let fcbit1719 = token_parser.fcbit1719();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                fcond2730,
                fcbit1719,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:83:1"]
#[derive(Clone, Debug)]
struct instructionVar137 {
    fcbit1719: TokenField_fcbit1719,
    R1115x2: TokenField_R1115x2,
    R0004x2: TokenField_R0004x2,
    R2731x2: TokenField_R2731x2,
}
impl instructionVar137 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 9usize] = [
            DisplayElement::Literal("cmovf.d"),
            DisplayElement::Literal(" "),
            self.fcbit1719.display(),
            DisplayElement::Literal(", "),
            self.R1115x2.display(),
            DisplayElement::Literal(", "),
            self.R0004x2.display(),
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115x2 = token_parser.R1115x2();
        let R0004x2 = token_parser.R0004x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 32 {
            return None;
        }
        if token_parser.op2020().disassembly() != 1 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R2731x2 = token_parser.R2731x2();
        let fcbit1719 = token_parser.fcbit1719();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115x2,
                R0004x2,
                R2731x2,
                fcbit1719,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:91:1"]
#[derive(Clone, Debug)]
struct instructionVar138 {
    fcbit1719: TokenField_fcbit1719,
    R1115: R1115,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar138 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmovf.s"),
            DisplayElement::Literal(" "),
            self.fcbit1719.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 32 {
            return None;
        }
        if token_parser.op2020().disassembly() != 0 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let fcbit1719 = token_parser.fcbit1719();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                R2731,
                fcbit1719,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:75:1"]
#[derive(Clone, Debug)]
struct instructionVar139 {
    R2731x2: TokenField_R2731x2,
    R1620x2: TokenField_R1620x2,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar139 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("mac"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
            DisplayElement::Literal(", "),
            self.R1620x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 30 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R1620x2 = token_parser.R1620x2();
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R1620x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:81:1"]
#[derive(Clone, Debug)]
struct instructionVar140 {
    R2731x2: TokenField_R2731x2,
    R1620x2: TokenField_R1620x2,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar140 {
    fn display_extend<T>(
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
            DisplayElement::Literal("macu"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(", "),
            self.R2731x2.display(),
            DisplayElement::Literal(", "),
            self.R1620x2.display(),
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 31 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R1620x2 = token_parser.R1620x2();
        let R2731x2 = token_parser.R2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R1620x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:146:1"]
#[derive(Clone, Debug)]
struct instructionVar141 {
    s1631: TokenField_s1631,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar141 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movea"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 49 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:152:1"]
#[derive(Clone, Debug)]
struct instructionVar142 {
    s1631: TokenField_s1631,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar142 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movhi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 50 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:226:1"]
#[derive(Clone, Debug)]
struct instructionVar143 {
    s1631: TokenField_s1631,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar143 {
    fn display_extend<T>(
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
            DisplayElement::Literal("satsubi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 51 {
            return None;
        }
        if token_parser.op1115().disassembly() == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:247:1"]
#[derive(Clone, Debug)]
struct instructionVar144 {
    R0004: R0004,
    R1115: R1115,
    c1720: c1720,
    R2731: R2731,
}
impl instructionVar144 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("cmov")];
        display.extend_from_slice(&extend);
        self.c1720.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 25 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            c1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                c1720,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:254:1"]
#[derive(Clone, Debug)]
struct instructionVar145 {
    s0004: TokenField_s0004,
    R1115: R1115,
    c1720: c1720,
    R2731: R2731,
}
impl instructionVar145 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("cmov")];
        display.extend_from_slice(&extend);
        self.c1720.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.s0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 24 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            c1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R1115,
                c1720,
                R2731,
                s0004,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:13:1"]
#[derive(Clone, Debug)]
struct instructionVar146 {
    R0004: R0004,
    R1115: R1115,
    c1720: c1720,
    R2731: R2731,
}
impl instructionVar146 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("adf")];
        display.extend_from_slice(&extend);
        self.c1720.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 29 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            c1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                c1720,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:24:1"]
#[derive(Clone, Debug)]
struct instructionVar147 {
    R0004: R0004,
    R1115: R1115,
    c1720: c1720,
    R2731: R2731,
}
impl instructionVar147 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("sbf")];
        display.extend_from_slice(&extend);
        self.c1720.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2126().disassembly() != 28 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            c1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                c1720,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:298:1"]
#[derive(Clone, Debug)]
struct instructionVar148 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
    reg4: reg4,
}
impl instructionVar148 {
    fn display_extend<T>(
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
            DisplayElement::Literal("maddf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.reg4.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2426().disassembly() != 5 {
            return None;
        }
        if token_parser.op2122().disassembly() != 0 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            reg4::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
                reg4,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:332:1"]
#[derive(Clone, Debug)]
struct instructionVar149 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
    reg4: reg4,
}
impl instructionVar149 {
    fn display_extend<T>(
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
            DisplayElement::Literal("msubf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.reg4.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2426().disassembly() != 5 {
            return None;
        }
        if token_parser.op2122().disassembly() != 1 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            reg4::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
                reg4,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:362:1"]
#[derive(Clone, Debug)]
struct instructionVar150 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
    reg4: reg4,
}
impl instructionVar150 {
    fn display_extend<T>(
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
            DisplayElement::Literal("nmaddf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.reg4.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2426().disassembly() != 5 {
            return None;
        }
        if token_parser.op2122().disassembly() != 2 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            reg4::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
                reg4,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc:368:1"]
#[derive(Clone, Debug)]
struct instructionVar151 {
    R0004: R0004,
    R1115: R1115,
    R2731: R2731,
    reg4: reg4,
}
impl instructionVar151 {
    fn display_extend<T>(
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
            DisplayElement::Literal("nmsubf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.reg4.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op2426().disassembly() != 5 {
            return None;
        }
        if token_parser.op2122().disassembly() != 3 {
            return None;
        }
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            reg4::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
                reg4,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:44:1"]
#[derive(Clone, Debug)]
struct instructionVar152 {
    s1631: TokenField_s1631,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar152 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ld.h"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 57 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:59:1"]
#[derive(Clone, Debug)]
struct instructionVar153 {
    s1731: TokenField_s1731,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar153 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp16: i64 = 0;
        disp16 = (self.s1731.disassembly() << (1u64 as i64));
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.hu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp16),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 63 {
            return None;
        }
        let mut disp16: i64 = 0;
        disp16 = (token_parser.s1731().disassembly() << (1u64 as i64));
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1616().disassembly() != 1 {
            return None;
        }
        let s1731 = token_parser.s1731();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:75:1"]
#[derive(Clone, Debug)]
struct instructionVar154 {
    s1731: TokenField_s1731,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar154 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp16: i64 = 0;
        disp16 = (self.s1731.disassembly() * (2u64 as i64));
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp16),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 57 {
            return None;
        }
        let mut disp16: i64 = 0;
        disp16 = (token_parser.s1731().disassembly() * (2u64 as i64));
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1616().disassembly() != 1 {
            return None;
        }
        let s1731 = token_parser.s1731();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:179:1"]
#[derive(Clone, Debug)]
struct instructionVar155 {
    s1631: TokenField_s1631,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar155 {
    fn display_extend<T>(
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
            DisplayElement::Literal("st.h"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 59 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:196:1"]
#[derive(Clone, Debug)]
struct instructionVar156 {
    s1731: TokenField_s1731,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar156 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp16: i64 = 0;
        disp16 = (self.s1731.disassembly() * (2u64 as i64));
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, disp16),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 59 {
            return None;
        }
        let mut disp16: i64 = 0;
        disp16 = (token_parser.s1731().disassembly() * (2u64 as i64));
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1616().disassembly() != 1 {
            return None;
        }
        let s1731 = token_parser.s1731();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:104:1"]
#[derive(Clone, Debug)]
struct instructionVar157 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: R0004,
}
impl instructionVar157 {
    fn display_extend<T>(
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
            DisplayElement::Literal("clr1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(", "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 62 {
            return None;
        }
        if token_parser.op1415().disassembly() != 2 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.op1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:121:1"]
#[derive(Clone, Debug)]
struct instructionVar158 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: R0004,
}
impl instructionVar158 {
    fn display_extend<T>(
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
            DisplayElement::Literal("not1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(", "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 62 {
            return None;
        }
        if token_parser.op1415().disassembly() != 1 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.op1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:138:1"]
#[derive(Clone, Debug)]
struct instructionVar159 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: R0004,
}
impl instructionVar159 {
    fn display_extend<T>(
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
            DisplayElement::Literal("set1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(", "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 62 {
            return None;
        }
        if token_parser.op1415().disassembly() != 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.op1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:155:1"]
#[derive(Clone, Debug)]
struct instructionVar160 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: R0004,
}
impl instructionVar160 {
    fn display_extend<T>(
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
            DisplayElement::Literal("tst1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(", "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 62 {
            return None;
        }
        if token_parser.op1415().disassembly() != 3 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.op1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:45:1"]
#[derive(Clone, Debug)]
struct instructionVar161 {
    s1631: TokenField_s1631,
    R1115: R1115,
    R0004: R0004,
}
impl instructionVar161 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mulhi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 55 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:94:1"]
#[derive(Clone, Debug)]
struct instructionVar162 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar162 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("add"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 14 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:101:1"]
#[derive(Clone, Debug)]
struct instructionVar163 {
    s0004: TokenField_s0004,
    R1115: R1115,
}
impl instructionVar163 {
    fn display_extend<T>(
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
            DisplayElement::Literal("add"),
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 18 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.s0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:108:1"]
#[derive(Clone, Debug)]
struct instructionVar164 {
    s1631: TokenField_s1631,
    R1115: R1115,
    R0004: R0004,
}
impl instructionVar164 {
    fn display_extend<T>(
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
            DisplayElement::Literal("addi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 48 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:115:1"]
#[derive(Clone, Debug)]
struct instructionVar165 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar165 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("cmp"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 15 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:121:1"]
#[derive(Clone, Debug)]
struct instructionVar166 {
    s0004: TokenField_s0004,
    R1115: R1115,
}
impl instructionVar166 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 19 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.s0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:127:1"]
#[derive(Clone, Debug)]
struct instructionVar167 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar167 {
    fn display_extend<T>(
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
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:51:1"]
#[derive(Clone, Debug)]
struct instructionVar168 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar168 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | (self.op2126.disassembly() << (1u64 as i64)));
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.h"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0515().disassembly() != 60 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | (token_parser.op2126().disassembly() << (1u64 as i64)));
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1620().disassembly() != 7 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.op2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2126,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:83:1"]
#[derive(Clone, Debug)]
struct instructionVar169 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar169 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | (self.op2126.disassembly() << (1u64 as i64)));
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0515().disassembly() != 60 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | (token_parser.op2126().disassembly() << (1u64 as i64)));
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1620().disassembly() != 9 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.op2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2126,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:187:1"]
#[derive(Clone, Debug)]
struct instructionVar170 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar170 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | (self.op2126.disassembly() << (1u64 as i64)));
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.h"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0515().disassembly() != 61 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | (token_parser.op2126().disassembly() << (1u64 as i64)));
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1620().disassembly() != 13 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.op2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2126,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:205:1"]
#[derive(Clone, Debug)]
struct instructionVar171 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar171 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | (self.op2126.disassembly() << (1u64 as i64)));
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0515().disassembly() != 60 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | (token_parser.op2126().disassembly() << (1u64 as i64)));
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1620().disassembly() != 15 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.op2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2126,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:20:1"]
#[derive(Clone, Debug)]
struct instructionVar172 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar172 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | self.op2026.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0515().disassembly() != 60 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | token_parser.op2026().disassembly());
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1619().disassembly() != 5 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.op2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2026,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:36:1"]
#[derive(Clone, Debug)]
struct instructionVar173 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar173 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | self.op2026.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.bu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0515().disassembly() != 61 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | token_parser.op2026().disassembly());
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1619().disassembly() != 5 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.op2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2026,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:67:1"]
#[derive(Clone, Debug)]
struct instructionVar174 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar174 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | self.op2026.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.hu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
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
        if token_parser.op0515().disassembly() != 61 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | token_parser.op2026().disassembly());
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1619().disassembly() != 7 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.op2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2026,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:170:1"]
#[derive(Clone, Debug)]
struct instructionVar175 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: R0004,
    R2731: R2731,
}
impl instructionVar175 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp23: i64 = 0;
        disp23 = ((self.s3247.disassembly() << (7u64 as i64))
            | self.op2026.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0515().disassembly() != 60 {
            return None;
        }
        let mut disp23: i64 = 0;
        disp23 = ((token_parser.s3247().disassembly() << (7u64 as i64))
            | token_parser.op2026().disassembly());
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1619().disassembly() != 13 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            R2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.op2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                op2026,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:139:1"]
#[derive(Clone, Debug)]
struct instructionVar176 {
    op3247: TokenField_op3247,
    op1631: TokenField_op1631,
    R0004: R0004,
}
impl instructionVar176 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut imm32: i64 = 0;
        imm32 = ((self.op3247.disassembly() << (16u64 as i64))
            | self.op1631.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mov"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, imm32),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
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
        if token_parser.op0515().disassembly() != 49 {
            return None;
        }
        let mut imm32: i64 = 0;
        imm32 = ((token_parser.op3247().disassembly() << (16u64 as i64))
            | token_parser.op1631().disassembly());
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op1631 = token_parser.op1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op3247 = token_parser.op3247();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1631,
                op3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:158:1"]
#[derive(Clone, Debug)]
struct instructionVar177 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar177 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("sub"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 13 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:165:1"]
#[derive(Clone, Debug)]
struct instructionVar178 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar178 {
    fn display_extend<T>(
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
            DisplayElement::Literal("subr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 12 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc:263:1"]
#[derive(Clone, Debug)]
struct instructionVar179 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar179 {
    fn display_extend<T>(
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
            DisplayElement::Literal("divh"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 2 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:13:1"]
#[derive(Clone, Debug)]
struct instructionVar180 {
    s1631: TokenField_s1631,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar180 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ld.b"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 56 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:149:1"]
#[derive(Clone, Debug)]
struct instructionVar181 {
    prep0105: TokenField_prep0105,
    DispList: DispList,
}
impl instructionVar181 {
    fn display_extend<T>(
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
            DisplayElement::Literal("dispose"),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.DispList.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep0615().disassembly() != 25 {
            return None;
        }
        if token_parser.prep1620().disassembly() != 0 {
            return None;
        }
        let DispList = if let Some((len, table)) =
            DispList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.prep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList, prep0105 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:222:1"]
#[derive(Clone, Debug)]
struct instructionVar182 {
    prep0105: TokenField_prep0105,
    PrepList: PrepList,
}
impl instructionVar182 {
    fn display_extend<T>(
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
            DisplayElement::Literal("prepare"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.prep0105.display()];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep0615().disassembly() != 30 {
            return None;
        }
        if token_parser.prep1620().disassembly() != 1 {
            return None;
        }
        let PrepList = if let Some((len, table)) =
            PrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.prep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList, prep0105 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:229:1"]
#[derive(Clone, Debug)]
struct instructionVar183 {
    prep0105: TokenField_prep0105,
    PrepList: PrepList,
}
impl instructionVar183 {
    fn display_extend<T>(
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
            DisplayElement::Literal("prepare"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(", "),
            self.prep0105.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::sp),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep0615().disassembly() != 30 {
            return None;
        }
        if token_parser.prep1620().disassembly() != 3 {
            return None;
        }
        let PrepList = if let Some((len, table)) =
            PrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.prep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList, prep0105 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:28:1"]
#[derive(Clone, Debug)]
struct instructionVar184 {
    s1731: TokenField_s1731,
    op0505: TokenField_op0505,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar184 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp16: i64 = 0;
        disp16 = ((self.s1731.disassembly() << (1u64 as i64))
            | self.op0505.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.bu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp16),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("], ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0610().disassembly() != 30 {
            return None;
        }
        let mut disp16: i64 = 0;
        disp16 = ((token_parser.s1731().disassembly() << (1u64 as i64))
            | token_parser.op0505().disassembly());
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0505 = token_parser.op0505();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1616().disassembly() != 1 {
            return None;
        }
        let s1731 = token_parser.s1731();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                op0505,
                s1731,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:57:1"]
#[derive(Clone, Debug)]
struct instructionVar185 {
    adr9: adr9,
}
impl instructionVar185 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("br"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr9.display_extend(
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
        if token_parser.op0710().disassembly() != 11 {
            return None;
        }
        if token_parser.op0003().disassembly() != 5 {
            return None;
        }
        let adr9 = if let Some((len, table)) =
            adr9::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { adr9 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:121:1"]
#[derive(Clone, Debug)]
struct instructionVar186 {
    op0106: TokenField_op0106,
    R1115: R1115,
}
impl instructionVar186 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp8: i64 = 0;
        disp8 = (self.op0106.disassembly() * (4u64 as i64));
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("sld.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp8),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("], "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0710().disassembly() != 10 {
            return None;
        }
        if token_parser.op0000().disassembly() != 0 {
            return None;
        }
        let mut disp8: i64 = 0;
        disp8 = (token_parser.op0106().disassembly() * (4u64 as i64));
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0106 = token_parser.op0106();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0106 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:153:1"]
#[derive(Clone, Debug)]
struct instructionVar187 {
    op0106: TokenField_op0106,
    R1115: R1115,
}
impl instructionVar187 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp8: i64 = 0;
        disp8 = (self.op0106.disassembly() * (4u64 as i64));
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sst.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, disp8),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("]"),
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
        if token_parser.op0710().disassembly() != 10 {
            return None;
        }
        if token_parser.op0000().disassembly() != 1 {
            return None;
        }
        let mut disp8: i64 = 0;
        disp8 = (token_parser.op0106().disassembly() * (4u64 as i64));
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0106 = token_parser.op0106();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0106 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:287:1"]
#[derive(Clone, Debug)]
struct instructionVar188 {
    op0004: TokenField_op0004,
    R1115: R1115,
}
impl instructionVar188 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sar"),
            DisplayElement::Literal(" "),
            self.op0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 21 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.op0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:317:1"]
#[derive(Clone, Debug)]
struct instructionVar189 {
    op0004: TokenField_op0004,
    R1115: R1115,
}
impl instructionVar189 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shl"),
            DisplayElement::Literal(" "),
            self.op0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 22 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.op0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:335:1"]
#[derive(Clone, Debug)]
struct instructionVar190 {
    op0004: TokenField_op0004,
    R1115: R1115,
}
impl instructionVar190 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shr"),
            DisplayElement::Literal(" "),
            self.op0004.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 20 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.op0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:42:1"]
#[derive(Clone, Debug)]
struct instructionVar191 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar191 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("and"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 10 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:56:1"]
#[derive(Clone, Debug)]
struct instructionVar192 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar192 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("not"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 1 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:63:1"]
#[derive(Clone, Debug)]
struct instructionVar193 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar193 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("or"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 8 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:77:1"]
#[derive(Clone, Debug)]
struct instructionVar194 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar194 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("tst"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 11 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:83:1"]
#[derive(Clone, Debug)]
struct instructionVar195 {
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar195 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("xor"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 9 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:94:1"]
#[derive(Clone, Debug)]
struct instructionVar196 {
    adr22: adr22,
}
impl instructionVar196 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("jr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr22.display_extend(
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
        if token_parser.op0615().disassembly() != 30 {
            return None;
        }
        let adr22 = if let Some((len, table)) =
            adr22::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { adr22 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:113:1"]
#[derive(Clone, Debug)]
struct instructionVar197 {
    op0005: TokenField_op0005,
}
impl instructionVar197 {
    fn display_extend<T>(
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
            DisplayElement::Literal("callt"),
            DisplayElement::Literal(" "),
            self.op0005.display(),
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
        if token_parser.op0615().disassembly() != 8 {
            return None;
        }
        let op0005 = token_parser.op0005();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0005 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:63:1"]
#[derive(Clone, Debug)]
struct instructionVar198 {
    R1115: R1115,
    adr22: adr22,
}
impl instructionVar198 {
    fn display_extend<T>(
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
            DisplayElement::Literal("jarl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.adr22.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        let mut block_0_len = 0u64 as u32;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.op0610().disassembly() != 30 {
                return None;
            }
            let R1115 = if let Some((len, table)) =
                R1115::parse(tokens, &mut context_instance, inst_start)
            {
                block_0_len = block_0_len.max(len as u32);
                table
            } else {
                return None;
            };
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((R1115), (), pattern_len))
        };
        let ((mut R1115), (), sub_len) =
            sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let adr22 = if let Some((len, table)) =
            adr22::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { R1115, adr22 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:91:1"]
#[derive(Clone, Debug)]
struct instructionVar199 {
    op0006: TokenField_op0006,
    R1115: R1115,
}
impl instructionVar199 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sld.b"),
            DisplayElement::Literal(" "),
            self.op0006.display(),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("], "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0710().disassembly() != 6 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.op0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:105:1"]
#[derive(Clone, Debug)]
struct instructionVar200 {
    op0006: TokenField_op0006,
    R1115: R1115,
}
impl instructionVar200 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp8: i64 = 0;
        disp8 = (self.op0006.disassembly() * (2u64 as i64));
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("sld.h"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, disp8),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("], "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0710().disassembly() != 8 {
            return None;
        }
        let mut disp8: i64 = 0;
        disp8 = (token_parser.op0006().disassembly() * (2u64 as i64));
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.op0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:136:1"]
#[derive(Clone, Debug)]
struct instructionVar201 {
    op0006: TokenField_op0006,
    R1115: R1115,
}
impl instructionVar201 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sst.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal(", "),
            self.op0006.display(),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("]"),
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
        if token_parser.op0710().disassembly() != 7 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.op0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:144:1"]
#[derive(Clone, Debug)]
struct instructionVar202 {
    op0006: TokenField_op0006,
    R1115: R1115,
}
impl instructionVar202 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut disp8: i64 = 0;
        disp8 = (self.op0006.disassembly() * (2u64 as i64));
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sst.h"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, disp8),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("]"),
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
        if token_parser.op0710().disassembly() != 9 {
            return None;
        }
        let mut disp8: i64 = 0;
        disp8 = (token_parser.op0006().disassembly() * (2u64 as i64));
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.op0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc:162:1"]
#[derive(Clone, Debug)]
struct instructionVar203 {
    s1631: TokenField_s1631,
    R0004: R0004,
    R1115: R1115,
}
impl instructionVar203 {
    fn display_extend<T>(
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
            DisplayElement::Literal("st.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", "),
            self.s1631.display(),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("]")];
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
        if token_parser.op0510().disassembly() != 58 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.s1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:49:1"]
#[derive(Clone, Debug)]
struct instructionVar204 {
    op1631: TokenField_op1631,
    R1115: R1115,
    R0004: R0004,
}
impl instructionVar204 {
    fn display_extend<T>(
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
            DisplayElement::Literal("andi"),
            DisplayElement::Literal(" "),
            self.op1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 54 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op1631 = token_parser.op1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                op1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:70:1"]
#[derive(Clone, Debug)]
struct instructionVar205 {
    op1631: TokenField_op1631,
    R1115: R1115,
    R0004: R0004,
}
impl instructionVar205 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ori"),
            DisplayElement::Literal(" "),
            self.op1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 52 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op1631 = token_parser.op1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                op1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc:90:1"]
#[derive(Clone, Debug)]
struct instructionVar206 {
    op1631: TokenField_op1631,
    R1115: R1115,
    R0004: R0004,
}
impl instructionVar206 {
    fn display_extend<T>(
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
            DisplayElement::Literal("xori"),
            DisplayElement::Literal(" "),
            self.op1631.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
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
        if token_parser.op0510().disassembly() != 53 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            R1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            R0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op1631 = token_parser.op1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                op1631,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:52:1"]
#[derive(Clone, Debug)]
struct instructionVar207 {
    c0003: c0003,
    adr9: adr9,
}
impl instructionVar207 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("b")];
        display.extend_from_slice(&extend);
        self.c0003.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr9.display_extend(
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
        if token_parser.op0710().disassembly() != 11 {
            return None;
        }
        let c0003 = if let Some((len, table)) =
            c0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let adr9 = if let Some((len, table)) =
            adr9::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { c0003, adr9 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:156:1"]
#[derive(Clone, Debug)]
struct instructionVar208 {
    prep0105: TokenField_prep0105,
    prep1620: TokenField_prep1620,
    DispList: DispList,
}
impl instructionVar208 {
    fn display_extend<T>(
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
            DisplayElement::Literal("dispose"),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        self.DispList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(", ["),
            self.prep1620.display(),
            DisplayElement::Literal("]"),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep0615().disassembly() != 25 {
            return None;
        }
        let DispList = if let Some((len, table)) =
            DispList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep1620 = token_parser.prep1620();
        let prep0105 = token_parser.prep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                DispList,
                prep1620,
                prep0105,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:237:1"]
#[derive(Clone, Debug)]
struct instructionVar209 {
    prep0105: TokenField_prep0105,
    s3247: TokenField_s3247,
    PrepList: PrepList,
}
impl instructionVar209 {
    fn display_extend<T>(
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
            DisplayElement::Literal("prepare"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(", "),
            self.prep0105.display(),
            DisplayElement::Literal(", "),
            self.s3247.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep0615().disassembly() != 30 {
            return None;
        }
        if token_parser.prep1620().disassembly() != 11 {
            return None;
        }
        let PrepList = if let Some((len, table)) =
            PrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.prep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                PrepList,
                prep0105,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:245:1"]
#[derive(Clone, Debug)]
struct instructionVar210 {
    prep0105: TokenField_prep0105,
    s3247: TokenField_s3247,
    PrepList: PrepList,
}
impl instructionVar210 {
    fn display_extend<T>(
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
            DisplayElement::Literal("prepare"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(", "),
            self.prep0105.display(),
            DisplayElement::Literal(", "),
            self.s3247.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep0615().disassembly() != 30 {
            return None;
        }
        if token_parser.prep1620().disassembly() != 19 {
            return None;
        }
        let PrepList = if let Some((len, table)) =
            PrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.prep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.s3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                PrepList,
                prep0105,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc:253:1"]
#[derive(Clone, Debug)]
struct instructionVar211 {
    prep0105: TokenField_prep0105,
    op4863: TokenField_op4863,
    op3247: TokenField_op3247,
    PrepList: PrepList,
}
impl instructionVar211 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut imm32: i64 = 0;
        imm32 = ((self.op4863.disassembly() << (16u64 as i64))
            | self.op3247.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("prepare"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(", "),
            self.prep0105.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Number(true, imm32),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep0615().disassembly() != 30 {
            return None;
        }
        if token_parser.prep1620().disassembly() != 27 {
            return None;
        }
        let mut imm32: i64 = 0;
        imm32 = ((token_parser.op4863().disassembly() << (16u64 as i64))
            | token_parser.op3247().disassembly());
        let PrepList = if let Some((len, table)) =
            PrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.prep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op3247 = token_parser.op3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op4863 = token_parser.op4863();
        pattern_len += block_2_len;
        tokens_current =
            &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                PrepList,
                prep0105,
                op3247,
                op4863,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum instruction {
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
    Var136(instructionVar136),
    Var137(instructionVar137),
    Var138(instructionVar138),
    Var139(instructionVar139),
    Var140(instructionVar140),
    Var141(instructionVar141),
    Var142(instructionVar142),
    Var143(instructionVar143),
    Var144(instructionVar144),
    Var145(instructionVar145),
    Var146(instructionVar146),
    Var147(instructionVar147),
    Var148(instructionVar148),
    Var149(instructionVar149),
    Var150(instructionVar150),
    Var151(instructionVar151),
    Var152(instructionVar152),
    Var153(instructionVar153),
    Var154(instructionVar154),
    Var155(instructionVar155),
    Var156(instructionVar156),
    Var157(instructionVar157),
    Var158(instructionVar158),
    Var159(instructionVar159),
    Var160(instructionVar160),
    Var161(instructionVar161),
    Var162(instructionVar162),
    Var163(instructionVar163),
    Var164(instructionVar164),
    Var165(instructionVar165),
    Var166(instructionVar166),
    Var167(instructionVar167),
    Var168(instructionVar168),
    Var169(instructionVar169),
    Var170(instructionVar170),
    Var171(instructionVar171),
    Var172(instructionVar172),
    Var173(instructionVar173),
    Var174(instructionVar174),
    Var175(instructionVar175),
    Var176(instructionVar176),
    Var177(instructionVar177),
    Var178(instructionVar178),
    Var179(instructionVar179),
    Var180(instructionVar180),
    Var181(instructionVar181),
    Var182(instructionVar182),
    Var183(instructionVar183),
    Var184(instructionVar184),
    Var185(instructionVar185),
    Var186(instructionVar186),
    Var187(instructionVar187),
    Var188(instructionVar188),
    Var189(instructionVar189),
    Var190(instructionVar190),
    Var191(instructionVar191),
    Var192(instructionVar192),
    Var193(instructionVar193),
    Var194(instructionVar194),
    Var195(instructionVar195),
    Var196(instructionVar196),
    Var197(instructionVar197),
    Var198(instructionVar198),
    Var199(instructionVar199),
    Var200(instructionVar200),
    Var201(instructionVar201),
    Var202(instructionVar202),
    Var203(instructionVar203),
    Var204(instructionVar204),
    Var205(instructionVar205),
    Var206(instructionVar206),
    Var207(instructionVar207),
    Var208(instructionVar208),
    Var209(instructionVar209),
    Var210(instructionVar210),
    Var211(instructionVar211),
}
impl instruction {
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
            Self::Var136(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var137(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var138(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var139(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var140(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var141(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var142(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var143(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var144(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var145(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var146(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var147(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var148(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var149(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var150(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var151(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var152(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var153(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var154(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var155(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var156(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var157(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var158(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var159(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var160(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var161(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var162(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var163(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var164(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var165(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var166(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var167(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var168(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var169(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var170(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var171(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var172(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var173(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var174(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var175(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var176(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var177(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var178(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var179(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var180(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var181(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var182(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var183(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var184(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var185(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var186(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var187(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var188(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var189(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var190(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var191(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var192(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var193(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var194(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var195(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var196(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var197(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var198(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var199(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var200(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var201(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var202(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var203(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var204(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var205(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var206(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var207(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var208(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var209(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var210(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var211(x) => x.display_extend(
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
        if let Some((inst_len, parsed)) = instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var136(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var137(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var138(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var139(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var140(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var141(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar142::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var142(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var143(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var144(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var145(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var146(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var147(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var148(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var149(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var150(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var151(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var152(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var153(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var154(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var155(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var156(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var157(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var158(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var159(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var160(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var161(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var162(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var163(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar164::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var164(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar165::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var165(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar166::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var166(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar167::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var167(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar168::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var168(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar169::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var169(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar170::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var170(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar171::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var171(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar172::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var172(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar173::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var173(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar174::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var174(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar175::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var175(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar176::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var176(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar177::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var177(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar178::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var178(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar179::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var179(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar180::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var180(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar181::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var181(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar182::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var182(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar183::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var183(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar184::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var184(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar185::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var185(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar186::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var186(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar187::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var187(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar188::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var188(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar189::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var189(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar190::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var190(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar191::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var191(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar192::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var192(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar193::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var193(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar194::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var194(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar195::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var195(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar196::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var196(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar197::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var197(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar198::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var198(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar199::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var199(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar200::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var200(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar201::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var201(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar202::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var202(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar203::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var203(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar204::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var204(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar205::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var205(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar206::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var206(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar207::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var207(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar208::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var208(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar209::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var209(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar210::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var210(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar211::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var211(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:6:1"]
#[derive(Clone, Debug)]
struct c0003Var0 {}
impl c0003Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("v")];
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
        if token_parser.op0003().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:7:1"]
#[derive(Clone, Debug)]
struct c0003Var1 {}
impl c0003Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nv")];
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
        if token_parser.op0003().disassembly() != 8 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:8:1"]
#[derive(Clone, Debug)]
struct c0003Var2 {}
impl c0003Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("c")];
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
        if token_parser.op0003().disassembly() != 1 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:9:1"]
#[derive(Clone, Debug)]
struct c0003Var3 {}
impl c0003Var3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nc")];
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
        if token_parser.op0003().disassembly() != 9 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:10:1"]
#[derive(Clone, Debug)]
struct c0003Var4 {}
impl c0003Var4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("e")];
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
        if token_parser.op0003().disassembly() != 2 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:11:1"]
#[derive(Clone, Debug)]
struct c0003Var5 {}
impl c0003Var5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ne")];
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
        if token_parser.op0003().disassembly() != 10 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:12:1"]
#[derive(Clone, Debug)]
struct c0003Var6 {}
impl c0003Var6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nh")];
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
        if token_parser.op0003().disassembly() != 3 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:13:1"]
#[derive(Clone, Debug)]
struct c0003Var7 {}
impl c0003Var7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("h")];
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
        if token_parser.op0003().disassembly() != 11 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:14:1"]
#[derive(Clone, Debug)]
struct c0003Var8 {}
impl c0003Var8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("n")];
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
        if token_parser.op0003().disassembly() != 4 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:15:1"]
#[derive(Clone, Debug)]
struct c0003Var9 {}
impl c0003Var9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("p")];
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
        if token_parser.op0003().disassembly() != 12 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:16:1"]
#[derive(Clone, Debug)]
struct c0003Var10 {}
impl c0003Var10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("t")];
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
        if token_parser.op0003().disassembly() != 5 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:17:1"]
#[derive(Clone, Debug)]
struct c0003Var11 {}
impl c0003Var11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("sa")];
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
        if token_parser.op0003().disassembly() != 13 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:18:1"]
#[derive(Clone, Debug)]
struct c0003Var12 {}
impl c0003Var12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("lt")];
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
        if token_parser.op0003().disassembly() != 6 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:19:1"]
#[derive(Clone, Debug)]
struct c0003Var13 {}
impl c0003Var13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ge")];
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
        if token_parser.op0003().disassembly() != 14 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:20:1"]
#[derive(Clone, Debug)]
struct c0003Var14 {}
impl c0003Var14 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("le")];
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
        if token_parser.op0003().disassembly() != 7 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:21:1"]
#[derive(Clone, Debug)]
struct c0003Var15 {}
impl c0003Var15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("gt")];
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
        if token_parser.op0003().disassembly() != 15 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum c0003 {
    Var0(c0003Var0),
    Var1(c0003Var1),
    Var2(c0003Var2),
    Var3(c0003Var3),
    Var4(c0003Var4),
    Var5(c0003Var5),
    Var6(c0003Var6),
    Var7(c0003Var7),
    Var8(c0003Var8),
    Var9(c0003Var9),
    Var10(c0003Var10),
    Var11(c0003Var11),
    Var12(c0003Var12),
    Var13(c0003Var13),
    Var14(c0003Var14),
    Var15(c0003Var15),
}
impl c0003 {
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
            c0003Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var8::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var9::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var10::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var11::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var12::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var13::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var14::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c0003Var15::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:23:1"]
#[derive(Clone, Debug)]
struct c1720Var0 {}
impl c1720Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("v")];
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
        if token_parser.op1720().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:24:1"]
#[derive(Clone, Debug)]
struct c1720Var1 {}
impl c1720Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nv")];
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
        if token_parser.op1720().disassembly() != 8 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:25:1"]
#[derive(Clone, Debug)]
struct c1720Var2 {}
impl c1720Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("c")];
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
        if token_parser.op1720().disassembly() != 1 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:26:1"]
#[derive(Clone, Debug)]
struct c1720Var3 {}
impl c1720Var3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nc")];
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
        if token_parser.op1720().disassembly() != 9 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:27:1"]
#[derive(Clone, Debug)]
struct c1720Var4 {}
impl c1720Var4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("e")];
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
        if token_parser.op1720().disassembly() != 2 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:28:1"]
#[derive(Clone, Debug)]
struct c1720Var5 {}
impl c1720Var5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ne")];
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
        if token_parser.op1720().disassembly() != 10 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:29:1"]
#[derive(Clone, Debug)]
struct c1720Var6 {}
impl c1720Var6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nh")];
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
        if token_parser.op1720().disassembly() != 3 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:30:1"]
#[derive(Clone, Debug)]
struct c1720Var7 {}
impl c1720Var7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("h")];
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
        if token_parser.op1720().disassembly() != 11 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:31:1"]
#[derive(Clone, Debug)]
struct c1720Var8 {}
impl c1720Var8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("n")];
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
        if token_parser.op1720().disassembly() != 4 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:32:1"]
#[derive(Clone, Debug)]
struct c1720Var9 {}
impl c1720Var9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("p")];
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
        if token_parser.op1720().disassembly() != 12 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:33:1"]
#[derive(Clone, Debug)]
struct c1720Var10 {}
impl c1720Var10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("t")];
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
        if token_parser.op1720().disassembly() != 5 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:34:1"]
#[derive(Clone, Debug)]
struct c1720Var11 {}
impl c1720Var11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("sa")];
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
        if token_parser.op1720().disassembly() != 13 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:35:1"]
#[derive(Clone, Debug)]
struct c1720Var12 {}
impl c1720Var12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("lt")];
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
        if token_parser.op1720().disassembly() != 6 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:36:1"]
#[derive(Clone, Debug)]
struct c1720Var13 {}
impl c1720Var13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ge")];
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
        if token_parser.op1720().disassembly() != 14 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:37:1"]
#[derive(Clone, Debug)]
struct c1720Var14 {}
impl c1720Var14 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("le")];
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
        if token_parser.op1720().disassembly() != 7 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc:38:1"]
#[derive(Clone, Debug)]
struct c1720Var15 {}
impl c1720Var15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("gt")];
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
        if token_parser.op1720().disassembly() != 15 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum c1720 {
    Var0(c1720Var0),
    Var1(c1720Var1),
    Var2(c1720Var2),
    Var3(c1720Var3),
    Var4(c1720Var4),
    Var5(c1720Var5),
    Var6(c1720Var6),
    Var7(c1720Var7),
    Var8(c1720Var8),
    Var9(c1720Var9),
    Var10(c1720Var10),
    Var11(c1720Var11),
    Var12(c1720Var12),
    Var13(c1720Var13),
    Var14(c1720Var14),
    Var15(c1720Var15),
}
impl c1720 {
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
            c1720Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var8::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var9::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var10::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var11::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var12::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var13::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var14::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) =
            c1720Var15::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:7:1"]
#[derive(Clone, Debug)]
struct R0004Var0 {
    _R0004: TokenField__R0004,
}
impl R0004Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self._R0004.display()];
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
        if token_parser._R0004().disassembly() != 0 {
            return None;
        }
        let _R0004 = token_parser._R0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R0004 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:8:1"]
#[derive(Clone, Debug)]
struct R0004Var1 {
    _R0004: TokenField__R0004,
}
impl R0004Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self._R0004.display()];
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
        let _R0004 = token_parser._R0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R0004 }))
    }
}
#[derive(Clone, Debug)]
enum R0004 {
    Var0(R0004Var0),
    Var1(R0004Var1),
}
impl R0004 {
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
            R0004Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            R0004Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:10:1"]
#[derive(Clone, Debug)]
struct R1115Var0 {
    _R1115: TokenField__R1115,
}
impl R1115Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self._R1115.display()];
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
        if token_parser._R1115().disassembly() != 0 {
            return None;
        }
        let _R1115 = token_parser._R1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R1115 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:11:1"]
#[derive(Clone, Debug)]
struct R1115Var1 {
    _R1115: TokenField__R1115,
}
impl R1115Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self._R1115.display()];
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
        let _R1115 = token_parser._R1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R1115 }))
    }
}
#[derive(Clone, Debug)]
enum R1115 {
    Var0(R1115Var0),
    Var1(R1115Var1),
}
impl R1115 {
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
            R1115Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            R1115Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:13:1"]
#[derive(Clone, Debug)]
struct R2731Var0 {
    _R2731: TokenField__R2731,
}
impl R2731Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self._R2731.display()];
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
        if token_parser._R2731().disassembly() != 0 {
            return None;
        }
        let _R2731 = token_parser._R2731();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R2731 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:14:1"]
#[derive(Clone, Debug)]
struct R2731Var1 {
    _R2731: TokenField__R2731,
}
impl R2731Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self._R2731.display()];
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
        let _R2731 = token_parser._R2731();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R2731 }))
    }
}
#[derive(Clone, Debug)]
enum R2731 {
    Var0(R2731Var0),
    Var1(R2731Var1),
}
impl R2731 {
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
            R2731Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            R2731Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:17:1"]
#[derive(Clone, Debug)]
struct adr9Var0 {
    s1115: TokenField_s1115,
    op0406: TokenField_op0406,
}
impl adr9Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut res: i64 = 0;
        res = (((self.s1115.disassembly() << (4u64 as i64))
            | (self.op0406.disassembly() << (1u64 as i64)))
            + i64::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, res)];
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
        let mut res: i64 = 0;
        res = (((token_parser.s1115().disassembly() << (4u64 as i64))
            | (token_parser.op0406().disassembly() << (1u64 as i64)))
            + i64::try_from(inst_start).unwrap());
        let op0406 = token_parser.op0406();
        let s1115 = token_parser.s1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0406, s1115 }))
    }
}
#[derive(Clone, Debug)]
enum adr9 {
    Var0(adr9Var0),
}
impl adr9 {
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
            adr9Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:23:1"]
#[derive(Clone, Debug)]
struct adr22Var0 {
    s0005: TokenField_s0005,
    op1631: TokenField_op1631,
}
impl adr22Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut res: i64 = 0;
        res = (((self.s0005.disassembly() << (16u64 as i64))
            | self.op1631.disassembly())
            + i64::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, res)];
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
        let mut res: i64 = 0;
        res = (((token_parser.s0005().disassembly() << (16u64 as i64))
            | token_parser.op1631().disassembly())
            + i64::try_from(inst_start).unwrap());
        let s0005 = token_parser.s0005();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let op1631 = token_parser.op1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { s0005, op1631 }))
    }
}
#[derive(Clone, Debug)]
enum adr22 {
    Var0(adr22Var0),
}
impl adr22 {
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
            adr22Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:29:1"]
#[derive(Clone, Debug)]
struct adr32Var0 {
    op3247: TokenField_op3247,
    op1631: TokenField_op1631,
}
impl adr32Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut res: i64 = 0;
        res = (((self.op3247.disassembly() << (16u64 as i64))
            | self.op1631.disassembly())
            + i64::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, res)];
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
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let mut res: i64 = 0;
        res = (((token_parser.op3247().disassembly() << (16u64 as i64))
            | token_parser.op1631().disassembly())
            + i64::try_from(inst_start).unwrap());
        let op1631 = token_parser.op1631();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op3247 = token_parser.op3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1631, op3247 }))
    }
}
#[derive(Clone, Debug)]
enum adr32 {
    Var0(adr32Var0),
}
impl adr32 {
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
            adr32Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:35:1"]
#[derive(Clone, Debug)]
struct adr32iVar0 {
    op3247: TokenField_op3247,
    op1631: TokenField_op1631,
}
impl adr32iVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut res: i64 = 0;
        res = ((self.op3247.disassembly() << (16u64 as i64))
            | self.op1631.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, res)];
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
        if token_parser.op1616().disassembly() != 0 {
            return None;
        }
        let mut res: i64 = 0;
        res = ((token_parser.op3247().disassembly() << (16u64 as i64))
            | token_parser.op1631().disassembly());
        let op1631 = token_parser.op1631();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op3247 = token_parser.op3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1631, op3247 }))
    }
}
#[derive(Clone, Debug)]
enum adr32i {
    Var0(adr32iVar0),
}
impl adr32i {
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
            adr32iVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:41:1"]
#[derive(Clone, Debug)]
struct reg4Var0 {
    op0_1720: TokenField_op0_1720,
}
impl reg4Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.op0_1720.display()];
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
        if token_parser.op2323().disassembly() != 0 {
            return None;
        }
        let op0_1720 = token_parser.op0_1720();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0_1720 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:42:1"]
#[derive(Clone, Debug)]
struct reg4Var1 {
    op1_1720: TokenField_op1_1720,
}
impl reg4Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.op1_1720.display()];
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
        if token_parser.op2323().disassembly() != 1 {
            return None;
        }
        let op1_1720 = token_parser.op1_1720();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1_1720 }))
    }
}
#[derive(Clone, Debug)]
enum reg4 {
    Var0(reg4Var0),
    Var1(reg4Var1),
}
impl reg4 {
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
            reg4Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            reg4Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:48:1"]
#[derive(Clone, Debug)]
struct PrepList20Var0 {}
impl PrepList20Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r20)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep27().disassembly() != 1 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:49:1"]
#[derive(Clone, Debug)]
struct PrepList20Var1 {}
impl PrepList20Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep27().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum PrepList20 {
    Var0(PrepList20Var0),
    Var1(PrepList20Var1),
}
impl PrepList20 {
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
        if let Some((inst_len, parsed)) = PrepList20Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList20Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:51:1"]
#[derive(Clone, Debug)]
struct PrepList21Var0 {}
impl PrepList21Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r21)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep26().disassembly() != 1 {
            return None;
        }
        if token_parser.prep27().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:52:1"]
#[derive(Clone, Debug)]
struct PrepList21Var1 {
    PrepList20: PrepList20,
}
impl PrepList21Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList20.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r21),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep26().disassembly() != 1 {
            return None;
        }
        let PrepList20 = if let Some((len, table)) =
            PrepList20::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList20 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:53:1"]
#[derive(Clone, Debug)]
struct PrepList21Var2 {
    PrepList20: PrepList20,
}
impl PrepList21Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList20.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep26().disassembly() != 0 {
            return None;
        }
        let PrepList20 = if let Some((len, table)) =
            PrepList20::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList20 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList21 {
    Var0(PrepList21Var0),
    Var1(PrepList21Var1),
    Var2(PrepList21Var2),
}
impl PrepList21 {
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
        if let Some((inst_len, parsed)) = PrepList21Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList21Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList21Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:55:1"]
#[derive(Clone, Debug)]
struct PrepList22Var0 {}
impl PrepList22Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r22)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep25().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2627().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:56:1"]
#[derive(Clone, Debug)]
struct PrepList22Var1 {
    PrepList21: PrepList21,
}
impl PrepList22Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList21.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r22),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep25().disassembly() != 1 {
            return None;
        }
        let PrepList21 = if let Some((len, table)) =
            PrepList21::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList21 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:57:1"]
#[derive(Clone, Debug)]
struct PrepList22Var2 {
    PrepList21: PrepList21,
}
impl PrepList22Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList21.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep25().disassembly() != 0 {
            return None;
        }
        let PrepList21 = if let Some((len, table)) =
            PrepList21::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList21 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList22 {
    Var0(PrepList22Var0),
    Var1(PrepList22Var1),
    Var2(PrepList22Var2),
}
impl PrepList22 {
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
        if let Some((inst_len, parsed)) = PrepList22Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList22Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList22Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:59:1"]
#[derive(Clone, Debug)]
struct PrepList23Var0 {}
impl PrepList23Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r23)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep24().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2527().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:60:1"]
#[derive(Clone, Debug)]
struct PrepList23Var1 {
    PrepList22: PrepList22,
}
impl PrepList23Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList22.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r23),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep24().disassembly() != 1 {
            return None;
        }
        let PrepList22 = if let Some((len, table)) =
            PrepList22::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList22 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:61:1"]
#[derive(Clone, Debug)]
struct PrepList23Var2 {
    PrepList22: PrepList22,
}
impl PrepList23Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList22.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep24().disassembly() != 0 {
            return None;
        }
        let PrepList22 = if let Some((len, table)) =
            PrepList22::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList22 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList23 {
    Var0(PrepList23Var0),
    Var1(PrepList23Var1),
    Var2(PrepList23Var2),
}
impl PrepList23 {
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
        if let Some((inst_len, parsed)) = PrepList23Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList23Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList23Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:63:1"]
#[derive(Clone, Debug)]
struct PrepList24Var0 {}
impl PrepList24Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r24)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep31().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2427().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:64:1"]
#[derive(Clone, Debug)]
struct PrepList24Var1 {
    PrepList23: PrepList23,
}
impl PrepList24Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList23.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r24),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep31().disassembly() != 1 {
            return None;
        }
        let PrepList23 = if let Some((len, table)) =
            PrepList23::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList23 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:65:1"]
#[derive(Clone, Debug)]
struct PrepList24Var2 {
    PrepList23: PrepList23,
}
impl PrepList24Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList23.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep31().disassembly() != 0 {
            return None;
        }
        let PrepList23 = if let Some((len, table)) =
            PrepList23::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList23 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList24 {
    Var0(PrepList24Var0),
    Var1(PrepList24Var1),
    Var2(PrepList24Var2),
}
impl PrepList24 {
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
        if let Some((inst_len, parsed)) = PrepList24Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList24Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList24Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:67:1"]
#[derive(Clone, Debug)]
struct PrepList25Var0 {}
impl PrepList25Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r25)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep30().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2427().disassembly() != 0 {
            return None;
        }
        if token_parser.prep31().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:68:1"]
#[derive(Clone, Debug)]
struct PrepList25Var1 {
    PrepList24: PrepList24,
}
impl PrepList25Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList24.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r25),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep30().disassembly() != 1 {
            return None;
        }
        let PrepList24 = if let Some((len, table)) =
            PrepList24::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList24 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:69:1"]
#[derive(Clone, Debug)]
struct PrepList25Var2 {
    PrepList24: PrepList24,
}
impl PrepList25Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList24.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep30().disassembly() != 0 {
            return None;
        }
        let PrepList24 = if let Some((len, table)) =
            PrepList24::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList24 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList25 {
    Var0(PrepList25Var0),
    Var1(PrepList25Var1),
    Var2(PrepList25Var2),
}
impl PrepList25 {
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
        if let Some((inst_len, parsed)) = PrepList25Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList25Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList25Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:71:1"]
#[derive(Clone, Debug)]
struct PrepList26Var0 {}
impl PrepList26Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r26)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep29().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2427().disassembly() != 0 {
            return None;
        }
        if token_parser.prep3031().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:72:1"]
#[derive(Clone, Debug)]
struct PrepList26Var1 {
    PrepList25: PrepList25,
}
impl PrepList26Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList25.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r26),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep29().disassembly() != 1 {
            return None;
        }
        let PrepList25 = if let Some((len, table)) =
            PrepList25::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList25 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:73:1"]
#[derive(Clone, Debug)]
struct PrepList26Var2 {
    PrepList25: PrepList25,
}
impl PrepList26Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList25.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep29().disassembly() != 0 {
            return None;
        }
        let PrepList25 = if let Some((len, table)) =
            PrepList25::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList25 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList26 {
    Var0(PrepList26Var0),
    Var1(PrepList26Var1),
    Var2(PrepList26Var2),
}
impl PrepList26 {
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
        if let Some((inst_len, parsed)) = PrepList26Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList26Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList26Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:75:1"]
#[derive(Clone, Debug)]
struct PrepList27Var0 {}
impl PrepList27Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r27)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep28().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2427().disassembly() != 0 {
            return None;
        }
        if token_parser.prep2931().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:76:1"]
#[derive(Clone, Debug)]
struct PrepList27Var1 {
    PrepList26: PrepList26,
}
impl PrepList27Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList26.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r27),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep28().disassembly() != 1 {
            return None;
        }
        let PrepList26 = if let Some((len, table)) =
            PrepList26::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList26 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:77:1"]
#[derive(Clone, Debug)]
struct PrepList27Var2 {
    PrepList26: PrepList26,
}
impl PrepList27Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList26.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep28().disassembly() != 0 {
            return None;
        }
        let PrepList26 = if let Some((len, table)) =
            PrepList26::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList26 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList27 {
    Var0(PrepList27Var0),
    Var1(PrepList27Var1),
    Var2(PrepList27Var2),
}
impl PrepList27 {
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
        if let Some((inst_len, parsed)) = PrepList27Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList27Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList27Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:79:1"]
#[derive(Clone, Debug)]
struct PrepList28Var0 {}
impl PrepList28Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r28)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep23().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2431().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:80:1"]
#[derive(Clone, Debug)]
struct PrepList28Var1 {
    PrepList27: PrepList27,
}
impl PrepList28Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList27.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r28),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep23().disassembly() != 1 {
            return None;
        }
        let PrepList27 = if let Some((len, table)) =
            PrepList27::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList27 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:81:1"]
#[derive(Clone, Debug)]
struct PrepList28Var2 {
    PrepList27: PrepList27,
}
impl PrepList28Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList27.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep23().disassembly() != 0 {
            return None;
        }
        let PrepList27 = if let Some((len, table)) =
            PrepList27::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList27 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList28 {
    Var0(PrepList28Var0),
    Var1(PrepList28Var1),
    Var2(PrepList28Var2),
}
impl PrepList28 {
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
        if let Some((inst_len, parsed)) = PrepList28Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList28Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList28Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:83:1"]
#[derive(Clone, Debug)]
struct PrepList29Var0 {}
impl PrepList29Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::r29)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep22().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2431().disassembly() != 0 {
            return None;
        }
        if token_parser.prep23().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:84:1"]
#[derive(Clone, Debug)]
struct PrepList29Var1 {
    PrepList28: PrepList28,
}
impl PrepList29Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList28.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r29),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep22().disassembly() != 1 {
            return None;
        }
        let PrepList28 = if let Some((len, table)) =
            PrepList28::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList28 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:85:1"]
#[derive(Clone, Debug)]
struct PrepList29Var2 {
    PrepList28: PrepList28,
}
impl PrepList29Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList28.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep22().disassembly() != 0 {
            return None;
        }
        let PrepList28 = if let Some((len, table)) =
            PrepList28::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList28 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList29 {
    Var0(PrepList29Var0),
    Var1(PrepList29Var1),
    Var2(PrepList29Var2),
}
impl PrepList29 {
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
        if let Some((inst_len, parsed)) = PrepList29Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList29Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList29Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:87:1"]
#[derive(Clone, Debug)]
struct PrepList30Var0 {}
impl PrepList30Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::ep)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep00().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2431().disassembly() != 0 {
            return None;
        }
        if token_parser.prep2223().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:88:1"]
#[derive(Clone, Debug)]
struct PrepList30Var1 {
    PrepList29: PrepList29,
}
impl PrepList30Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList29.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::ep),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep00().disassembly() != 1 {
            return None;
        }
        let PrepList29 = if let Some((len, table)) =
            PrepList29::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList29 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:89:1"]
#[derive(Clone, Debug)]
struct PrepList30Var2 {
    PrepList29: PrepList29,
}
impl PrepList30Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.PrepList29.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep00().disassembly() != 0 {
            return None;
        }
        let PrepList29 = if let Some((len, table)) =
            PrepList29::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList29 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList30 {
    Var0(PrepList30Var0),
    Var1(PrepList30Var1),
    Var2(PrepList30Var2),
}
impl PrepList30 {
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
        if let Some((inst_len, parsed)) = PrepList30Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList30Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = PrepList30Var2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:91:1"]
#[derive(Clone, Debug)]
struct PrepListVar0 {}
impl PrepListVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("{ "),
            DisplayElement::Register(Register::lp),
            DisplayElement::Literal(" }"),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep21().disassembly() != 1 {
            return None;
        }
        if token_parser.prep2431().disassembly() != 0 {
            return None;
        }
        if token_parser.prep2223().disassembly() != 0 {
            return None;
        }
        if token_parser.prep00().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:92:1"]
#[derive(Clone, Debug)]
struct PrepListVar1 {
    PrepList30: PrepList30,
}
impl PrepListVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("{ ")];
        display.extend_from_slice(&extend);
        self.PrepList30.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::lp),
            DisplayElement::Literal(" }"),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep21().disassembly() != 1 {
            return None;
        }
        let PrepList30 = if let Some((len, table)) =
            PrepList30::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList30 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:93:1"]
#[derive(Clone, Debug)]
struct PrepListVar2 {
    PrepList30: PrepList30,
}
impl PrepListVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("{ ")];
        display.extend_from_slice(&extend);
        self.PrepList30.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" }")];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep21().disassembly() != 0 {
            return None;
        }
        let PrepList30 = if let Some((len, table)) =
            PrepList30::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { PrepList30 }))
    }
}
#[derive(Clone, Debug)]
enum PrepList {
    Var0(PrepListVar0),
    Var1(PrepListVar1),
    Var2(PrepListVar2),
}
impl PrepList {
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
            PrepListVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            PrepListVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            PrepListVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:97:1"]
#[derive(Clone, Debug)]
struct DispList31Var0 {}
impl DispList31Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::lp)];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep21().disassembly() != 1 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:98:1"]
#[derive(Clone, Debug)]
struct DispList31Var1 {}
impl DispList31Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep21().disassembly() != 0 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum DispList31 {
    Var0(DispList31Var0),
    Var1(DispList31Var1),
}
impl DispList31 {
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
        if let Some((inst_len, parsed)) = DispList31Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList31Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:100:1"]
#[derive(Clone, Debug)]
struct DispList30Var0 {
    DispList31: DispList31,
}
impl DispList30Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList31.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep00().disassembly() != 1 {
            return None;
        }
        let DispList31 = if let Some((len, table)) =
            DispList31::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList31 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:101:1"]
#[derive(Clone, Debug)]
struct DispList30Var1 {
    DispList31: DispList31,
}
impl DispList30Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList31.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep00().disassembly() != 0 {
            return None;
        }
        let DispList31 = if let Some((len, table)) =
            DispList31::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList31 }))
    }
}
#[derive(Clone, Debug)]
enum DispList30 {
    Var0(DispList30Var0),
    Var1(DispList30Var1),
}
impl DispList30 {
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
        if let Some((inst_len, parsed)) = DispList30Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList30Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:103:1"]
#[derive(Clone, Debug)]
struct DispList29Var0 {
    DispList30: DispList30,
}
impl DispList29Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r29),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList30.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep22().disassembly() != 1 {
            return None;
        }
        let DispList30 = if let Some((len, table)) =
            DispList30::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList30 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:104:1"]
#[derive(Clone, Debug)]
struct DispList29Var1 {
    DispList30: DispList30,
}
impl DispList29Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList30.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep22().disassembly() != 0 {
            return None;
        }
        let DispList30 = if let Some((len, table)) =
            DispList30::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList30 }))
    }
}
#[derive(Clone, Debug)]
enum DispList29 {
    Var0(DispList29Var0),
    Var1(DispList29Var1),
}
impl DispList29 {
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
        if let Some((inst_len, parsed)) = DispList29Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList29Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:106:1"]
#[derive(Clone, Debug)]
struct DispList28Var0 {
    DispList29: DispList29,
}
impl DispList28Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r28),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList29.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep23().disassembly() != 1 {
            return None;
        }
        let DispList29 = if let Some((len, table)) =
            DispList29::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList29 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:107:1"]
#[derive(Clone, Debug)]
struct DispList28Var1 {
    DispList29: DispList29,
}
impl DispList28Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList29.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep23().disassembly() != 0 {
            return None;
        }
        let DispList29 = if let Some((len, table)) =
            DispList29::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList29 }))
    }
}
#[derive(Clone, Debug)]
enum DispList28 {
    Var0(DispList28Var0),
    Var1(DispList28Var1),
}
impl DispList28 {
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
        if let Some((inst_len, parsed)) = DispList28Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList28Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:109:1"]
#[derive(Clone, Debug)]
struct DispList27Var0 {
    DispList28: DispList28,
}
impl DispList27Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r27),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList28.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep28().disassembly() != 1 {
            return None;
        }
        let DispList28 = if let Some((len, table)) =
            DispList28::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList28 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:110:1"]
#[derive(Clone, Debug)]
struct DispList27Var1 {
    DispList28: DispList28,
}
impl DispList27Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList28.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep28().disassembly() != 0 {
            return None;
        }
        let DispList28 = if let Some((len, table)) =
            DispList28::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList28 }))
    }
}
#[derive(Clone, Debug)]
enum DispList27 {
    Var0(DispList27Var0),
    Var1(DispList27Var1),
}
impl DispList27 {
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
        if let Some((inst_len, parsed)) = DispList27Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList27Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:112:1"]
#[derive(Clone, Debug)]
struct DispList26Var0 {
    DispList27: DispList27,
}
impl DispList26Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r26),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList27.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep29().disassembly() != 1 {
            return None;
        }
        let DispList27 = if let Some((len, table)) =
            DispList27::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList27 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:113:1"]
#[derive(Clone, Debug)]
struct DispList26Var1 {
    DispList27: DispList27,
}
impl DispList26Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList27.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep29().disassembly() != 0 {
            return None;
        }
        let DispList27 = if let Some((len, table)) =
            DispList27::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList27 }))
    }
}
#[derive(Clone, Debug)]
enum DispList26 {
    Var0(DispList26Var0),
    Var1(DispList26Var1),
}
impl DispList26 {
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
        if let Some((inst_len, parsed)) = DispList26Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList26Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:115:1"]
#[derive(Clone, Debug)]
struct DispList25Var0 {
    DispList26: DispList26,
}
impl DispList25Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r25),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList26.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep30().disassembly() != 1 {
            return None;
        }
        let DispList26 = if let Some((len, table)) =
            DispList26::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList26 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:116:1"]
#[derive(Clone, Debug)]
struct DispList25Var1 {
    DispList26: DispList26,
}
impl DispList25Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList26.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep30().disassembly() != 0 {
            return None;
        }
        let DispList26 = if let Some((len, table)) =
            DispList26::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList26 }))
    }
}
#[derive(Clone, Debug)]
enum DispList25 {
    Var0(DispList25Var0),
    Var1(DispList25Var1),
}
impl DispList25 {
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
        if let Some((inst_len, parsed)) = DispList25Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList25Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:118:1"]
#[derive(Clone, Debug)]
struct DispList24Var0 {
    DispList25: DispList25,
}
impl DispList24Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r24),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList25.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep31().disassembly() != 1 {
            return None;
        }
        let DispList25 = if let Some((len, table)) =
            DispList25::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList25 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:119:1"]
#[derive(Clone, Debug)]
struct DispList24Var1 {
    DispList25: DispList25,
}
impl DispList24Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList25.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep31().disassembly() != 0 {
            return None;
        }
        let DispList25 = if let Some((len, table)) =
            DispList25::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList25 }))
    }
}
#[derive(Clone, Debug)]
enum DispList24 {
    Var0(DispList24Var0),
    Var1(DispList24Var1),
}
impl DispList24 {
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
        if let Some((inst_len, parsed)) = DispList24Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList24Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:121:1"]
#[derive(Clone, Debug)]
struct DispList23Var0 {
    DispList24: DispList24,
}
impl DispList23Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r23),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList24.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep24().disassembly() != 1 {
            return None;
        }
        let DispList24 = if let Some((len, table)) =
            DispList24::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList24 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:122:1"]
#[derive(Clone, Debug)]
struct DispList23Var1 {
    DispList24: DispList24,
}
impl DispList23Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList24.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep24().disassembly() != 0 {
            return None;
        }
        let DispList24 = if let Some((len, table)) =
            DispList24::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList24 }))
    }
}
#[derive(Clone, Debug)]
enum DispList23 {
    Var0(DispList23Var0),
    Var1(DispList23Var1),
}
impl DispList23 {
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
        if let Some((inst_len, parsed)) = DispList23Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList23Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:124:1"]
#[derive(Clone, Debug)]
struct DispList22Var0 {
    DispList23: DispList23,
}
impl DispList22Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r22),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList23.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep25().disassembly() != 1 {
            return None;
        }
        let DispList23 = if let Some((len, table)) =
            DispList23::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList23 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:125:1"]
#[derive(Clone, Debug)]
struct DispList22Var1 {
    DispList23: DispList23,
}
impl DispList22Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList23.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep25().disassembly() != 0 {
            return None;
        }
        let DispList23 = if let Some((len, table)) =
            DispList23::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList23 }))
    }
}
#[derive(Clone, Debug)]
enum DispList22 {
    Var0(DispList22Var0),
    Var1(DispList22Var1),
}
impl DispList22 {
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
        if let Some((inst_len, parsed)) = DispList22Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList22Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:127:1"]
#[derive(Clone, Debug)]
struct DispList21Var0 {
    DispList22: DispList22,
}
impl DispList21Var0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r21),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList22.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep26().disassembly() != 1 {
            return None;
        }
        let DispList22 = if let Some((len, table)) =
            DispList22::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList22 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:128:1"]
#[derive(Clone, Debug)]
struct DispList21Var1 {
    DispList22: DispList22,
}
impl DispList21Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.DispList22.display_extend(
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep26().disassembly() != 0 {
            return None;
        }
        let DispList22 = if let Some((len, table)) =
            DispList22::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList22 }))
    }
}
#[derive(Clone, Debug)]
enum DispList21 {
    Var0(DispList21Var0),
    Var1(DispList21Var1),
}
impl DispList21 {
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
        if let Some((inst_len, parsed)) = DispList21Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = DispList21Var1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:130:1"]
#[derive(Clone, Debug)]
struct DispListVar0 {
    DispList21: DispList21,
}
impl DispListVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("{ "),
            DisplayElement::Register(Register::r20),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList21.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" }")];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep27().disassembly() != 1 {
            return None;
        }
        let DispList21 = if let Some((len, table)) =
            DispList21::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList21 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc:131:1"]
#[derive(Clone, Debug)]
struct DispListVar1 {
    DispList21: DispList21,
}
impl DispListVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("{ ")];
        display.extend_from_slice(&extend);
        self.DispList21.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" }")];
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.prep27().disassembly() != 0 {
            return None;
        }
        let DispList21 = if let Some((len, table)) =
            DispList21::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { DispList21 }))
    }
}
#[derive(Clone, Debug)]
enum DispList {
    Var0(DispListVar0),
    Var1(DispListVar1),
}
impl DispList {
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
            DispListVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DispListVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
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
        instruction::parse(tokens, context, inst_start)?;
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
