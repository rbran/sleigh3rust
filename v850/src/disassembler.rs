use sleigh4rust::*;
pub type AddrType = u32;
pub trait GlobalSetTrait {}
#[derive(Default)]
pub struct GlobalSetDefault;
impl GlobalSetTrait for GlobalSetDefault {}
pub trait ContextTrait: Default {}
#[derive(Debug, Clone, Copy, Default)]
pub struct SpacesStruct {}
impl ContextTrait for SpacesStruct {}
fn meaning_number<T>(hex: bool, num: T) -> DisplayElement
where
    i128: TryFrom<T>,
    <i128 as TryFrom<T>>::Error: core::fmt::Debug,
{
    DisplayElement::Number(hex, i128::try_from(num).unwrap())
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
struct TokenParser<const LEN: usize>([u8; LEN]);
impl<const LEN: usize> MemoryRead for TokenParser<LEN> {
    type AddressType = usize;
    fn read(
        &self,
        addr: Self::AddressType,
        buf: &mut [u8],
    ) -> Result<(), MemoryReadError<Self::AddressType>> {
        let end = addr + buf.len();
        self.0
            .get(addr..end)
            .map(|src| buf.copy_from_slice(src))
            .ok_or(MemoryReadError::UnableToReadMemory(addr, end))
    }
}
impl<const LEN: usize> TokenParser<LEN> {
    fn new(data: &[u8]) -> Option<Self> {
        let token_slice: &[u8] = data.get(..LEN)?;
        let token_data = <[u8; LEN]>::try_from(token_slice).unwrap();
        Some(Self(token_data))
    }
    fn TokenFieldop0000(&self) -> TokenField_op0000 {
        let inner_value = self.read_u8::<false>(0, 0, 1).unwrap();
        TokenField_op0000(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0003(&self) -> TokenField_op0003 {
        let inner_value = self.read_u8::<false>(0, 0, 4).unwrap();
        TokenField_op0003(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0004(&self) -> TokenField_op0004 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_op0004(u8::try_from(inner_value).unwrap())
    }
    fn TokenField_R0004(&self) -> TokenField__R0004 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField__R0004(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldSR0004(&self) -> TokenField_SR0004 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_SR0004(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldR0004x2(&self) -> TokenField_R0004x2 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_R0004x2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFields0004(&self) -> TokenField_s0004 {
        let inner_value = self.read_i8::<false>(0, 0, 5).unwrap();
        TokenField_s0004(i8::try_from(inner_value).unwrap())
    }
    fn TokenFields0005(&self) -> TokenField_s0005 {
        let inner_value = self.read_i8::<false>(0, 0, 6).unwrap();
        TokenField_s0005(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0005(&self) -> TokenField_op0005 {
        let inner_value = self.read_u8::<false>(0, 0, 6).unwrap();
        TokenField_op0005(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0006(&self) -> TokenField_op0006 {
        let inner_value = self.read_u8::<false>(0, 0, 7).unwrap();
        TokenField_op0006(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0010(&self) -> TokenField_op0010 {
        let inner_value = self.read_u16::<false>(0, 0, 11).unwrap();
        TokenField_op0010(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0015(&self) -> TokenField_op0015 {
        let inner_value = self.read_u16::<false>(0, 0, 16).unwrap();
        TokenField_op0015(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0106(&self) -> TokenField_op0106 {
        let inner_value = self.read_u8::<false>(0, 1, 6).unwrap();
        TokenField_op0106(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0406(&self) -> TokenField_op0406 {
        let inner_value = self.read_u8::<false>(0, 4, 3).unwrap();
        TokenField_op0406(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0410(&self) -> TokenField_op0410 {
        let inner_value = self.read_u16::<false>(0, 4, 7).unwrap();
        TokenField_op0410(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0505(&self) -> TokenField_op0505 {
        let inner_value = self.read_u8::<false>(0, 5, 1).unwrap();
        TokenField_op0505(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0510(&self) -> TokenField_op0510 {
        let inner_value = self.read_u16::<false>(0, 5, 6).unwrap();
        TokenField_op0510(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0515(&self) -> TokenField_op0515 {
        let inner_value = self.read_u16::<false>(0, 5, 11).unwrap();
        TokenField_op0515(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0610(&self) -> TokenField_op0610 {
        let inner_value = self.read_u16::<false>(0, 6, 5).unwrap();
        TokenField_op0610(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0615(&self) -> TokenField_op0615 {
        let inner_value = self.read_u16::<false>(0, 6, 10).unwrap();
        TokenField_op0615(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0710(&self) -> TokenField_op0710 {
        let inner_value = self.read_u16::<false>(0, 7, 4).unwrap();
        TokenField_op0710(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1113(&self) -> TokenField_op1113 {
        let inner_value = self.read_u8::<false>(1, 3, 3).unwrap();
        TokenField_op1113(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1114(&self) -> TokenField_op1114 {
        let inner_value = self.read_u8::<false>(1, 3, 4).unwrap();
        TokenField_op1114(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1115(&self) -> TokenField_op1115 {
        let inner_value = self.read_u8::<false>(1, 3, 5).unwrap();
        TokenField_op1115(u8::try_from(inner_value).unwrap())
    }
    fn TokenField_R1115(&self) -> TokenField__R1115 {
        let inner_value = self.read_u8::<false>(1, 3, 5).unwrap();
        TokenField__R1115(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldSR1115(&self) -> TokenField_SR1115 {
        let inner_value = self.read_u8::<false>(1, 3, 5).unwrap();
        TokenField_SR1115(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldR1115x2(&self) -> TokenField_R1115x2 {
        let inner_value = self.read_u8::<false>(1, 3, 5).unwrap();
        TokenField_R1115x2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFields1115(&self) -> TokenField_s1115 {
        let inner_value = self.read_i8::<false>(1, 3, 5).unwrap();
        TokenField_s1115(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1415(&self) -> TokenField_op1415 {
        let inner_value = self.read_u8::<false>(1, 6, 2).unwrap();
        TokenField_op1415(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1515(&self) -> TokenField_op1515 {
        let inner_value = self.read_u8::<false>(1, 7, 1).unwrap();
        TokenField_op1515(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1616(&self) -> TokenField_op1616 {
        let inner_value = self.read_u8::<false>(0, 0, 1).unwrap();
        TokenField_op1616(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1617(&self) -> TokenField_op1617 {
        let inner_value = self.read_u8::<false>(0, 0, 2).unwrap();
        TokenField_op1617(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1619(&self) -> TokenField_op1619 {
        let inner_value = self.read_u8::<false>(0, 0, 4).unwrap();
        TokenField_op1619(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1620(&self) -> TokenField_op1620 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_op1620(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldR1620(&self) -> TokenField_R1620 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_R1620(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldR1620x2(&self) -> TokenField_R1620x2 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_R1620x2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1626(&self) -> TokenField_op1626 {
        let inner_value = self.read_u16::<false>(0, 0, 11).unwrap();
        TokenField_op1626(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1631(&self) -> TokenField_op1631 {
        let inner_value = self.read_u16::<false>(0, 0, 16).unwrap();
        TokenField_op1631(u16::try_from(inner_value).unwrap())
    }
    fn TokenFields1631(&self) -> TokenField_s1631 {
        let inner_value = self.read_i16::<false>(0, 0, 16).unwrap();
        TokenField_s1631(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldfcbit1719(&self) -> TokenField_fcbit1719 {
        let inner_value = self.read_u8::<false>(0, 1, 3).unwrap();
        TokenField_fcbit1719(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1720(&self) -> TokenField_op1720 {
        let inner_value = self.read_u8::<false>(0, 1, 4).unwrap();
        TokenField_op1720(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0_1720(&self) -> TokenField_op0_1720 {
        let inner_value = self.read_u8::<false>(0, 1, 4).unwrap();
        TokenField_op0_1720(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1_1720(&self) -> TokenField_op1_1720 {
        let inner_value = self.read_u8::<false>(0, 1, 4).unwrap();
        TokenField_op1_1720(u8::try_from(inner_value).unwrap())
    }
    fn TokenFields1731(&self) -> TokenField_s1731 {
        let inner_value = self.read_i16::<false>(0, 1, 15).unwrap();
        TokenField_s1731(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1821(&self) -> TokenField_op1821 {
        let inner_value = self.read_u8::<false>(0, 2, 4).unwrap();
        TokenField_op1821(u8::try_from(inner_value).unwrap())
    }
    fn TokenFields1821(&self) -> TokenField_s1821 {
        let inner_value = self.read_i8::<false>(0, 2, 4).unwrap();
        TokenField_s1821(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2020(&self) -> TokenField_op2020 {
        let inner_value = self.read_u8::<false>(0, 4, 1).unwrap();
        TokenField_op2020(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2026(&self) -> TokenField_op2026 {
        let inner_value = self.read_u16::<false>(0, 4, 7).unwrap();
        TokenField_op2026(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2122(&self) -> TokenField_op2122 {
        let inner_value = self.read_u8::<false>(0, 5, 2).unwrap();
        TokenField_op2122(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2126(&self) -> TokenField_op2126 {
        let inner_value = self.read_u16::<false>(0, 5, 6).unwrap();
        TokenField_op2126(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2226(&self) -> TokenField_op2226 {
        let inner_value = self.read_u16::<false>(0, 6, 5).unwrap();
        TokenField_op2226(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2323(&self) -> TokenField_op2323 {
        let inner_value = self.read_u8::<false>(0, 7, 1).unwrap();
        TokenField_op2323(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2426(&self) -> TokenField_op2426 {
        let inner_value = self.read_u8::<false>(1, 0, 3).unwrap();
        TokenField_op2426(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2729(&self) -> TokenField_op2729 {
        let inner_value = self.read_u8::<false>(1, 3, 3).unwrap();
        TokenField_op2729(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfcond2730(&self) -> TokenField_fcond2730 {
        let inner_value = self.read_u8::<false>(1, 3, 4).unwrap();
        TokenField_fcond2730(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2731(&self) -> TokenField_op2731 {
        let inner_value = self.read_u8::<false>(1, 3, 5).unwrap();
        TokenField_op2731(u8::try_from(inner_value).unwrap())
    }
    fn TokenField_R2731(&self) -> TokenField__R2731 {
        let inner_value = self.read_u8::<false>(1, 3, 5).unwrap();
        TokenField__R2731(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldR2731x2(&self) -> TokenField_R2731x2 {
        let inner_value = self.read_u8::<false>(1, 3, 5).unwrap();
        TokenField_R2731x2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop3031(&self) -> TokenField_op3031 {
        let inner_value = self.read_u8::<false>(1, 6, 2).unwrap();
        TokenField_op3031(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop3131(&self) -> TokenField_op3131 {
        let inner_value = self.read_u8::<false>(1, 7, 1).unwrap();
        TokenField_op3131(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop3247(&self) -> TokenField_op3247 {
        let inner_value = self.read_u16::<false>(0, 0, 16).unwrap();
        TokenField_op3247(u16::try_from(inner_value).unwrap())
    }
    fn TokenFields3247(&self) -> TokenField_s3247 {
        let inner_value = self.read_i16::<false>(0, 0, 16).unwrap();
        TokenField_s3247(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop4863(&self) -> TokenField_op4863 {
        let inner_value = self.read_u16::<false>(0, 0, 16).unwrap();
        TokenField_op4863(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep00(&self) -> TokenField_prep00 {
        let inner_value = self.read_u8::<false>(0, 0, 1).unwrap();
        TokenField_prep00(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep0105(&self) -> TokenField_prep0105 {
        let inner_value = self.read_u8::<false>(0, 1, 5).unwrap();
        TokenField_prep0105(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep0615(&self) -> TokenField_prep0615 {
        let inner_value = self.read_u16::<false>(0, 6, 10).unwrap();
        TokenField_prep0615(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep1620(&self) -> TokenField_prep1620 {
        let inner_value = self.read_u8::<false>(2, 0, 5).unwrap();
        TokenField_prep1620(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep21(&self) -> TokenField_prep21 {
        let inner_value = self.read_u8::<false>(2, 5, 1).unwrap();
        TokenField_prep21(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep22(&self) -> TokenField_prep22 {
        let inner_value = self.read_u8::<false>(2, 6, 1).unwrap();
        TokenField_prep22(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep2223(&self) -> TokenField_prep2223 {
        let inner_value = self.read_u16::<false>(2, 6, 10).unwrap();
        TokenField_prep2223(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep23(&self) -> TokenField_prep23 {
        let inner_value = self.read_u8::<false>(2, 7, 1).unwrap();
        TokenField_prep23(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep24(&self) -> TokenField_prep24 {
        let inner_value = self.read_u8::<false>(3, 0, 1).unwrap();
        TokenField_prep24(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep2431(&self) -> TokenField_prep2431 {
        let inner_value = self.read_u8::<false>(3, 0, 8).unwrap();
        TokenField_prep2431(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep25(&self) -> TokenField_prep25 {
        let inner_value = self.read_u8::<false>(3, 1, 1).unwrap();
        TokenField_prep25(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep26(&self) -> TokenField_prep26 {
        let inner_value = self.read_u8::<false>(3, 2, 1).unwrap();
        TokenField_prep26(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep27(&self) -> TokenField_prep27 {
        let inner_value = self.read_u8::<false>(3, 3, 1).unwrap();
        TokenField_prep27(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep28(&self) -> TokenField_prep28 {
        let inner_value = self.read_u8::<false>(3, 4, 1).unwrap();
        TokenField_prep28(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep29(&self) -> TokenField_prep29 {
        let inner_value = self.read_u8::<false>(3, 5, 1).unwrap();
        TokenField_prep29(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep2931(&self) -> TokenField_prep2931 {
        let inner_value = self.read_u8::<false>(3, 5, 3).unwrap();
        TokenField_prep2931(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep3031(&self) -> TokenField_prep3031 {
        let inner_value = self.read_u8::<false>(3, 6, 2).unwrap();
        TokenField_prep3031(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep2427(&self) -> TokenField_prep2427 {
        let inner_value = self.read_u8::<false>(3, 0, 4).unwrap();
        TokenField_prep2427(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep2527(&self) -> TokenField_prep2527 {
        let inner_value = self.read_u8::<false>(3, 1, 3).unwrap();
        TokenField_prep2527(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep2627(&self) -> TokenField_prep2627 {
        let inner_value = self.read_u8::<false>(3, 2, 2).unwrap();
        TokenField_prep2627(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep30(&self) -> TokenField_prep30 {
        let inner_value = self.read_u8::<false>(3, 6, 1).unwrap();
        TokenField_prep30(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldprep31(&self) -> TokenField_prep31 {
        let inner_value = self.read_u8::<false>(3, 7, 1).unwrap();
        TokenField_prep31(u8::try_from(inner_value).unwrap())
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
    Number(bool, i128),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:142:1, end:142:2))"]
#[derive(Clone, Debug)]
struct di_instructionVar0 {}
impl di_instructionVar0 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 2016i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 352i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:166:1, end:166:2))"]
#[derive(Clone, Debug)]
struct ei_instructionVar1 {}
impl ei_instructionVar1 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 34784i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 352i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct halt_instructionVar2 {}
impl halt_instructionVar2 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 2016i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 288i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:7:1, end:7:2))"]
#[derive(Clone, Debug)]
struct absf_d_instructionVar3 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl absf_d_instructionVar3 {
    fn display_extend<T>(
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
            DisplayElement::Literal("absf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 0i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 24i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct absf_s_instructionVar4 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl absf_s_instructionVar4 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 0i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 8i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:31:1, end:31:2))"]
#[derive(Clone, Debug)]
struct ceilf_dl_instructionVar5 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl ceilf_dl_instructionVar5 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ceilf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 2i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:38:1, end:38:2))"]
#[derive(Clone, Debug)]
struct ceilf_dul_instructionVar6 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl ceilf_dul_instructionVar6 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ceilf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 18i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:45:1, end:45:2))"]
#[derive(Clone, Debug)]
struct ceilf_duw_instructionVar7 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl ceilf_duw_instructionVar7 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ceilf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 18i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:51:1, end:51:2))"]
#[derive(Clone, Debug)]
struct ceilf_dw_instructionVar8 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl ceilf_dw_instructionVar8 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ceilf.dw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 2i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:57:1, end:57:2))"]
#[derive(Clone, Debug)]
struct ceilf_sl_instructionVar9 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl ceilf_sl_instructionVar9 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 2i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:64:1, end:64:2))"]
#[derive(Clone, Debug)]
struct ceilf_sul_instructionVar10 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl ceilf_sul_instructionVar10 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 18i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:71:1, end:71:2))"]
#[derive(Clone, Debug)]
struct ceilf_sul_instructionVar11 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl ceilf_sul_instructionVar11 {
    fn display_extend<T>(
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
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 18i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:77:1, end:77:2))"]
#[derive(Clone, Debug)]
struct ceilf_sw_instructionVar12 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl ceilf_sw_instructionVar12 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 2i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:126:1, end:126:2))"]
#[derive(Clone, Debug)]
struct cvtf_dl_instructionVar13 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl cvtf_dl_instructionVar13 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cvtf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 4i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:132:1, end:132:2))"]
#[derive(Clone, Debug)]
struct cvtf_ds_instructionVar14 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl cvtf_ds_instructionVar14 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cvtf.ds"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 3i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 18i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:138:1, end:138:2))"]
#[derive(Clone, Debug)]
struct cvtf_dul_instructionVar15 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl cvtf_dul_instructionVar15 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cvtf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 20i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:144:1, end:144:2))"]
#[derive(Clone, Debug)]
struct cvtf_duw_instructionVar16 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl cvtf_duw_instructionVar16 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cvtf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 20i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:150:1, end:150:2))"]
#[derive(Clone, Debug)]
struct cvtf_sw_instructionVar17 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl cvtf_sw_instructionVar17 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cvtf.sw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 4i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:156:1, end:156:2))"]
#[derive(Clone, Debug)]
struct cvtf_ls_instructionVar18 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl cvtf_ls_instructionVar18 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cvtf.ls"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 18i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct cvtf_ls_instructionVar19 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl cvtf_ls_instructionVar19 {
    fn display_extend<T>(
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
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 2i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:168:1, end:168:2))"]
#[derive(Clone, Debug)]
struct cvtf_sd_instructionVar20 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl cvtf_sd_instructionVar20 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 2i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 18i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:174:1, end:174:2))"]
#[derive(Clone, Debug)]
struct cvtf_sl_instructionVar21 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl cvtf_sl_instructionVar21 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 4i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:180:1, end:180:2))"]
#[derive(Clone, Debug)]
struct cvtf_sul_instructionVar22 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl cvtf_sul_instructionVar22 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 20i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:186:1, end:186:2))"]
#[derive(Clone, Debug)]
struct cvtf_suw_instructionVar23 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_suw_instructionVar23 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 20i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:192:1, end:192:2))"]
#[derive(Clone, Debug)]
struct cvtf_sw_instructionVar24 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_sw_instructionVar24 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 4i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:198:1, end:198:2))"]
#[derive(Clone, Debug)]
struct cvtf_uls_instructionVar25 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl cvtf_uls_instructionVar25 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cvtf.uls"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 17i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 18i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct cvtf_uls_instructionVar26 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl cvtf_uls_instructionVar26 {
    fn display_extend<T>(
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
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 17i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 2i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:210:1, end:210:2))"]
#[derive(Clone, Debug)]
struct cvtf_uwd_instructionVar27 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl cvtf_uwd_instructionVar27 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 16i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 18i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:216:1, end:216:2))"]
#[derive(Clone, Debug)]
struct cvtf_uws_instructionVar28 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_uws_instructionVar28 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 16i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 2i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct cvtf_wd_instructionVar29 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl cvtf_wd_instructionVar29 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 0i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 18i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:228:1, end:228:2))"]
#[derive(Clone, Debug)]
struct cvtf_ws_instructionVar30 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_ws_instructionVar30 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 0i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 2i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:246:1, end:246:2))"]
#[derive(Clone, Debug)]
struct floorf_dl_instructionVar31 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl floorf_dl_instructionVar31 {
    fn display_extend<T>(
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
            DisplayElement::Literal("floorf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 3i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:253:1, end:253:2))"]
#[derive(Clone, Debug)]
struct floorf_dul_instructionVar32 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl floorf_dul_instructionVar32 {
    fn display_extend<T>(
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
            DisplayElement::Literal("floorf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 19i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:260:1, end:260:2))"]
#[derive(Clone, Debug)]
struct floorf_duw_instructionVar33 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl floorf_duw_instructionVar33 {
    fn display_extend<T>(
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
            DisplayElement::Literal("floorf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 19i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:266:1, end:266:2))"]
#[derive(Clone, Debug)]
struct floorf_dw_instructionVar34 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl floorf_dw_instructionVar34 {
    fn display_extend<T>(
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
            DisplayElement::Literal("floorf.dw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 3i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:272:1, end:272:2))"]
#[derive(Clone, Debug)]
struct floorf_sl_instructionVar35 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl floorf_sl_instructionVar35 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 3i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:279:1, end:279:2))"]
#[derive(Clone, Debug)]
struct floorf_sul_instructionVar36 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl floorf_sul_instructionVar36 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 19i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:286:1, end:286:2))"]
#[derive(Clone, Debug)]
struct floorf_suw_instructionVar37 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl floorf_suw_instructionVar37 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 19i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:292:1, end:292:2))"]
#[derive(Clone, Debug)]
struct floorf_suw_instructionVar38 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl floorf_suw_instructionVar38 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 3i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:374:1, end:374:2))"]
#[derive(Clone, Debug)]
struct negf_d_instructionVar39 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl negf_d_instructionVar39 {
    fn display_extend<T>(
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
            DisplayElement::Literal("negf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 24i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:380:1, end:380:2))"]
#[derive(Clone, Debug)]
struct negf_s_instructionVar40 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl negf_s_instructionVar40 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 8i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:398:1, end:398:2))"]
#[derive(Clone, Debug)]
struct recipf_d_instructionVar41 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl recipf_d_instructionVar41 {
    fn display_extend<T>(
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
            DisplayElement::Literal("recipf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 30i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct recipf_s_instructionVar42 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl recipf_s_instructionVar42 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 14i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:410:1, end:410:2))"]
#[derive(Clone, Debug)]
struct rsqrtf_d_instructionVar43 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl rsqrtf_d_instructionVar43 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rsqrtf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 2i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 30i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:416:1, end:416:2))"]
#[derive(Clone, Debug)]
struct rsqrtf_s_instructionVar44 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl rsqrtf_s_instructionVar44 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 2i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 14i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:422:1, end:422:2))"]
#[derive(Clone, Debug)]
struct sqrtf_d_instructionVar45 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl sqrtf_d_instructionVar45 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sqrtf.d"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 0i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 30i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:428:1, end:428:2))"]
#[derive(Clone, Debug)]
struct sqrtf_s_instructionVar46 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sqrtf_s_instructionVar46 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 0i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 14i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:453:1, end:453:2))"]
#[derive(Clone, Debug)]
struct trncf_dl_instructionVar47 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl trncf_dl_instructionVar47 {
    fn display_extend<T>(
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
            DisplayElement::Literal("trncf.dl"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:459:1, end:459:2))"]
#[derive(Clone, Debug)]
struct trncf_dul_instructionVar48 {
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl trncf_dul_instructionVar48 {
    fn display_extend<T>(
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
            DisplayElement::Literal("trncf.dul"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 17i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:465:1, end:465:2))"]
#[derive(Clone, Debug)]
struct trncf_duw_instructionVar49 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl trncf_duw_instructionVar49 {
    fn display_extend<T>(
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
            DisplayElement::Literal("trncf.duw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 17i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:471:1, end:471:2))"]
#[derive(Clone, Debug)]
struct trncf_dw_instructionVar50 {
    R1115x2: TokenField_R1115x2,
    R2731: TableR2731,
}
impl trncf_dw_instructionVar50 {
    fn display_extend<T>(
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
            DisplayElement::Literal("trncf.dw"),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:477:1, end:477:2))"]
#[derive(Clone, Debug)]
struct trncf_sl_instructionVar51 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl trncf_sl_instructionVar51 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:483:1, end:483:2))"]
#[derive(Clone, Debug)]
struct trncf_sul_instructionVar52 {
    R2731x2: TokenField_R2731x2,
    R1115: TableR1115,
}
impl trncf_sul_instructionVar52 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 17i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:489:1, end:489:2))"]
#[derive(Clone, Debug)]
struct trncf_suw_instructionVar53 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl trncf_suw_instructionVar53 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 17i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:495:1, end:495:2))"]
#[derive(Clone, Debug)]
struct trncf_sw_instructionVar54 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl trncf_sw_instructionVar54 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 1i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 34i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:221:1, end:221:2))"]
#[derive(Clone, Debug)]
struct bsh_instructionVar55 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl bsh_instructionVar55 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 834i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:235:1, end:235:2))"]
#[derive(Clone, Debug)]
struct bsw_instructionVar56 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl bsw_instructionVar56 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 832i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:261:1, end:261:2))"]
#[derive(Clone, Debug)]
struct hsh_instructionVar57 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl hsh_instructionVar57 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 838i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:271:1, end:271:2))"]
#[derive(Clone, Debug)]
struct hsw_instructionVar58 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl hsw_instructionVar58 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 836i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:299:1, end:299:2))"]
#[derive(Clone, Debug)]
struct sasf_instructionVar59 {
    c0003: Tablec0003,
    R1115: TableR1115,
}
impl sasf_instructionVar59 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0410().disassembly() != 126i128 {
            return None;
        }
        let c0003 = if let Some((len, table)) =
            Tablec0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 512i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { c0003, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:305:1, end:305:2))"]
#[derive(Clone, Debug)]
struct setf_instructionVar60 {
    c0003: Tablec0003,
    R1115: TableR1115,
}
impl setf_instructionVar60 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0410().disassembly() != 126i128 {
            return None;
        }
        let c0003 = if let Some((len, table)) =
            Tablec0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { c0003, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:133:1, end:133:2))"]
#[derive(Clone, Debug)]
struct ctret_instructionVar61 {}
impl ctret_instructionVar61 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 63i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 324i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:173:1, end:173:2))"]
#[derive(Clone, Debug)]
struct eiret_instructionVar62 {}
impl eiret_instructionVar62 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 63i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 328i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:181:1, end:181:2))"]
#[derive(Clone, Debug)]
struct feret_instructionVar63 {}
impl feret_instructionVar63 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 63i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 330i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:262:1, end:262:2))"]
#[derive(Clone, Debug)]
struct reti_instructionVar64 {}
impl reti_instructionVar64 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 63i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 320i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:300:1, end:300:2))"]
#[derive(Clone, Debug)]
struct rie_instructionVar65 {
    op1115: TokenField_op1115,
    op0003: TokenField_op0003,
}
impl rie_instructionVar65 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rie"),
            DisplayElement::Literal(" "),
            self.op1115.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0410().disassembly() != 127i128 {
            return None;
        }
        let op1115 = token_parser.TokenFieldop1115();
        let op0003 = token_parser.TokenFieldop0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1115, op0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:361:1, end:361:2))"]
#[derive(Clone, Debug)]
struct trap_instructionVar66 {
    op0004: TokenField_op0004,
}
impl trap_instructionVar66 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 63i128 {
            return None;
        }
        let op0004 = token_parser.TokenFieldop0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 256i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:281:1, end:281:2))"]
#[derive(Clone, Debug)]
struct sar_instructionVar67 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl sar_instructionVar67 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 160i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:311:1, end:311:2))"]
#[derive(Clone, Debug)]
struct shl_instructionVar68 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl shl_instructionVar68 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 192i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:329:1, end:329:2))"]
#[derive(Clone, Debug)]
struct shr_instructionVar69 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl shr_instructionVar69 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 128i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:113:1, end:113:2))"]
#[derive(Clone, Debug)]
struct clr1_instructionVar70 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl clr1_instructionVar70 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 228i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:130:1, end:130:2))"]
#[derive(Clone, Debug)]
struct not1_instructionVar71 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl not1_instructionVar71 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 226i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:147:1, end:147:2))"]
#[derive(Clone, Debug)]
struct set1_instructionVar72 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl set1_instructionVar72 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 224i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:163:1, end:163:2))"]
#[derive(Clone, Debug)]
struct tst1_instructionVar73 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl tst1_instructionVar73 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("["),
        ];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1631().disassembly() != 230i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct sch0l_instructionVar74 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch0l_instructionVar74 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 868i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:21:1, end:21:2))"]
#[derive(Clone, Debug)]
struct sch0r_instructionVar75 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch0r_instructionVar75 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 864i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:29:1, end:29:2))"]
#[derive(Clone, Debug)]
struct sch1l_instructionVar76 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch1l_instructionVar76 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 870i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:37:1, end:37:2))"]
#[derive(Clone, Debug)]
struct sch1r_instructionVar77 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch1r_instructionVar77 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0010().disassembly() != 2016i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 866i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:210:1, end:210:2))"]
#[derive(Clone, Debug)]
struct ldsr_instructionVar78 {
    SR1115: TokenField_SR1115,
    R0004: TableR0004,
}
impl ldsr_instructionVar78 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.SR1115.display(),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let SR1115 = token_parser.TokenFieldSR1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 32i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, SR1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:312:1, end:312:2))"]
#[derive(Clone, Debug)]
struct stsr_instructionVar79 {
    SR0004: TokenField_SR0004,
    R1115: TableR1115,
}
impl stsr_instructionVar79 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stsr"),
            DisplayElement::Literal(" "),
            self.SR0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let SR0004 = token_parser.TokenFieldSR0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1631().disassembly() != 64i128 {
            return None;
        }
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, SR0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:345:1, end:345:2))"]
#[derive(Clone, Debug)]
struct syscall_instructionVar80 {
    op2729: TokenField_op2729,
    op0004: TokenField_op0004,
}
impl syscall_instructionVar80 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_vector8: i128 = 0;
        calc_vector8 = (u32::try_from(5i128)
            .ok()
            .map(|shl| self.op2729.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op0004.disassembly());
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("syscall"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_vector8),
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
        let mut calc_vector8: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 1727i128 {
            return None;
        }
        let op0004 = token_parser.TokenFieldop0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop3031().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop1626().disassembly() != 352i128 {
            return None;
        }
        calc_vector8 = (u32::try_from(5i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldop2729()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop0004().disassembly());
        let op2729 = token_parser.TokenFieldop2729();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0004, op2729 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:16:1, end:16:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar81 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mul_instructionVar81 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 544i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:51:1, end:51:2))"]
#[derive(Clone, Debug)]
struct mulu_instructionVar82 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mulu_instructionVar82 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 546i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:199:1, end:199:2))"]
#[derive(Clone, Debug)]
struct satadd_instructionVar83 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl satadd_instructionVar83 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 954i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:217:1, end:217:2))"]
#[derive(Clone, Debug)]
struct satsub_instructionVar84 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl satsub_instructionVar84 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 922i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:251:1, end:251:2))"]
#[derive(Clone, Debug)]
struct div_instructionVar85 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl div_instructionVar85 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 704i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:272:1, end:272:2))"]
#[derive(Clone, Debug)]
struct divh_instructionVar86 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divh_instructionVar86 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 640i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:284:1, end:284:2))"]
#[derive(Clone, Debug)]
struct divhu_instructionVar87 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divhu_instructionVar87 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 642i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:296:1, end:296:2))"]
#[derive(Clone, Debug)]
struct divu_instructionVar88 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divu_instructionVar88 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 706i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:315:1, end:315:2))"]
#[derive(Clone, Debug)]
struct divq_instructionVar89 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divq_instructionVar89 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 764i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct divqu_instructionVar90 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divqu_instructionVar90 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 766i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:19:1, end:19:2))"]
#[derive(Clone, Debug)]
struct addf_d_instructionVar91 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl addf_d_instructionVar91 {
    fn display_extend<T>(
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
            DisplayElement::Literal("addf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        let R0004x2 = token_parser.TokenFieldR0004x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 16i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:25:1, end:25:2))"]
#[derive(Clone, Debug)]
struct addf_s_instructionVar92 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl addf_s_instructionVar92 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct divf_s_instructionVar93 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl divf_s_instructionVar93 {
    fn display_extend<T>(
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
            DisplayElement::Literal("divf.s"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004x2 = token_parser.TokenFieldR0004x2();
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 30i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:240:1, end:240:2))"]
#[derive(Clone, Debug)]
struct divf_s_instructionVar94 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divf_s_instructionVar94 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 14i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:298:1, end:298:2))"]
#[derive(Clone, Debug)]
struct fmaf_s_instructionVar95 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl fmaf_s_instructionVar95 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmaf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 39i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:304:1, end:304:2))"]
#[derive(Clone, Debug)]
struct fmsf_s_instructionVar96 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl fmsf_s_instructionVar96 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmsf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 39i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 2i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:310:1, end:310:2))"]
#[derive(Clone, Debug)]
struct fnmaf_s_instructionVar97 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl fnmaf_s_instructionVar97 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fnmaf.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 39i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:316:1, end:316:2))"]
#[derive(Clone, Debug)]
struct fnmfs_s_instructionVar98 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl fnmfs_s_instructionVar98 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fnmfs.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 39i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 6i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:328:1, end:328:2))"]
#[derive(Clone, Debug)]
struct maxf_d_instructionVar99 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl maxf_d_instructionVar99 {
    fn display_extend<T>(
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
            DisplayElement::Literal("maxf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004x2 = token_parser.TokenFieldR0004x2();
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 24i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:335:1, end:335:2))"]
#[derive(Clone, Debug)]
struct maxf_s_instructionVar100 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl maxf_s_instructionVar100 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 8i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:342:1, end:342:2))"]
#[derive(Clone, Debug)]
struct minf_d_instructionVar101 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl minf_d_instructionVar101 {
    fn display_extend<T>(
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
            DisplayElement::Literal("minf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004x2 = token_parser.TokenFieldR0004x2();
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 26i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:349:1, end:349:2))"]
#[derive(Clone, Debug)]
struct minf_s_instructionVar102 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl minf_s_instructionVar102 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 10i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:362:1, end:362:2))"]
#[derive(Clone, Debug)]
struct mulf_d_instructionVar103 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl mulf_d_instructionVar103 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mulf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004x2 = token_parser.TokenFieldR0004x2();
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 20i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:368:1, end:368:2))"]
#[derive(Clone, Debug)]
struct mulf_s_instructionVar104 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mulf_s_instructionVar104 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 4i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:434:1, end:434:2))"]
#[derive(Clone, Debug)]
struct subf_d_instructionVar105 {
    R0004x2: TokenField_R0004x2,
    R1115x2: TokenField_R1115x2,
    R2731x2: TokenField_R2731x2,
}
impl subf_d_instructionVar105 {
    fn display_extend<T>(
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
            DisplayElement::Literal("subf.d"),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004x2 = token_parser.TokenFieldR0004x2();
        let R1115x2 = token_parser.TokenFieldR1115x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 18i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:440:1, end:440:2))"]
#[derive(Clone, Debug)]
struct subf_s_instructionVar106 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl subf_s_instructionVar106 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 35i128 {
            return None;
        }
        if token_parser.TokenFieldop1620().disassembly() != 2i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:293:1, end:293:2))"]
#[derive(Clone, Debug)]
struct sar_instructionVar107 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sar_instructionVar107 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 162i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:323:1, end:323:2))"]
#[derive(Clone, Debug)]
struct shl_instructionVar108 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl shl_instructionVar108 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 194i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:341:1, end:341:2))"]
#[derive(Clone, Debug)]
struct shr_instructionVar109 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl shr_instructionVar109 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 130i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:123:1, end:123:2))"]
#[derive(Clone, Debug)]
struct caxi_instructionVar110 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl caxi_instructionVar110 {
    fn display_extend<T>(
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
            DisplayElement::Literal("caxi"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1626().disassembly() != 238i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:24:1, end:24:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar111 {
    s1821: TokenField_s1821,
    op0004: TokenField_op0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mul_instructionVar111 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_imm9: i128 = 0;
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .map(|shl| self.s1821.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op0004.disassembly());
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mul"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_imm9),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        let mut calc_imm9: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.TokenFieldop0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2226().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldop1617().disassembly() != 0i128 {
            return None;
        }
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields1821()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop0004().disassembly());
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s1821 = token_parser.TokenFields1821();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:446:1, end:446:2))"]
#[derive(Clone, Debug)]
struct trfsr_instructionVar112 {
    fcbit1719: TokenField_fcbit1719,
}
impl trfsr_instructionVar112 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop1115().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2731().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop2126().disassembly() != 32i128 {
            return None;
        }
        if token_parser.TokenFieldop2020().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let fcbit1719 = token_parser.TokenFieldfcbit1719();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fcbit1719 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:51:1, end:51:2))"]
#[derive(Clone, Debug)]
struct ld_h_instructionVar113 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_h_instructionVar113 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| self.op2126.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0));
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.h"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 60i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1620().disassembly() != 7i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.TokenFieldop2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldop2126()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0));
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:83:1, end:83:2))"]
#[derive(Clone, Debug)]
struct ld_w_instructionVar114 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_w_instructionVar114 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| self.op2126.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0));
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 60i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1620().disassembly() != 9i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.TokenFieldop2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldop2126()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0));
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:187:1, end:187:2))"]
#[derive(Clone, Debug)]
struct st_h_instructionVar115 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl st_h_instructionVar115 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| self.op2126.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0));
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.h"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 61i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1620().disassembly() != 13i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.TokenFieldop2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldop2126()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0));
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:205:1, end:205:2))"]
#[derive(Clone, Debug)]
struct st_w_instructionVar116 {
    s3247: TokenField_s3247,
    op2126: TokenField_op2126,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl st_w_instructionVar116 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| self.op2126.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0));
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 60i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1620().disassembly() != 15i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2126 = token_parser.TokenFieldop2126();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldop2126()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0));
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:20:1, end:20:2))"]
#[derive(Clone, Debug)]
struct ld_b_instructionVar117 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_b_instructionVar117 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op2026.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 60i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1619().disassembly() != 5i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.TokenFieldop2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop2026().disassembly());
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:36:1, end:36:2))"]
#[derive(Clone, Debug)]
struct ld_bu_instructionVar118 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_bu_instructionVar118 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op2026.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.bu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 61i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1619().disassembly() != 5i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.TokenFieldop2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop2026().disassembly());
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:67:1, end:67:2))"]
#[derive(Clone, Debug)]
struct ld_hu_instructionVar119 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_hu_instructionVar119 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op2026.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.hu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 61i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1619().disassembly() != 7i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.TokenFieldop2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop2026().disassembly());
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:170:1, end:170:2))"]
#[derive(Clone, Debug)]
struct st_b_instructionVar120 {
    s3247: TokenField_s3247,
    op2026: TokenField_op2026,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl st_b_instructionVar120 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| self.s3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op2026.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp23),
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
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 60i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1619().disassembly() != 13i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op2026 = token_parser.TokenFieldop2026();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop2026().disassembly());
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:77:1, end:77:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar121 {
    R0004: TableR0004,
}
impl jmp_instructionVar121 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0515().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() != 31i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:81:1, end:81:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar122 {
    R0004: TableR0004,
}
impl jmp_instructionVar122 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0515().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldop0004().disassembly() == 31i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:100:1, end:100:2))"]
#[derive(Clone, Debug)]
struct jr_instructionVar123 {
    adr32: Tableadr32,
}
impl jr_instructionVar123 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 736i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u32;
        let adr32 = if let Some((len, table)) =
            Tableadr32::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:189:1, end:189:2))"]
#[derive(Clone, Debug)]
struct fetrap_instructionVar124 {
    op1114: TokenField_op1114,
}
impl fetrap_instructionVar124 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0010().disassembly() != 64i128 {
            return None;
        }
        if token_parser.TokenFieldop1515().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let op1114 = token_parser.TokenFieldop1114();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1114 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:216:1, end:216:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar125 {}
impl nop_instructionVar125 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:288:1, end:288:2))"]
#[derive(Clone, Debug)]
struct rie_instructionVar126 {}
impl rie_instructionVar126 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 64i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct synce_instructionVar127 {}
impl synce_instructionVar127 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 29i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:333:1, end:333:2))"]
#[derive(Clone, Debug)]
struct syncm_instructionVar128 {}
impl syncm_instructionVar128 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 30i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:339:1, end:339:2))"]
#[derive(Clone, Debug)]
struct syncp_instructionVar129 {}
impl syncp_instructionVar129 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0015().disassembly() != 31i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:33:1, end:33:2))"]
#[derive(Clone, Debug)]
struct mulh_instructionVar130 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl mulh_instructionVar130 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:59:1, end:59:2))"]
#[derive(Clone, Debug)]
struct mulu_instructionVar131 {
    op1821: TokenField_op1821,
    op0004: TokenField_op0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mulu_instructionVar131 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_imm9: i128 = 0;
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .map(|shl| self.op1821.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op0004.disassembly());
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mulu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_imm9),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        let mut calc_imm9: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.TokenFieldop0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2226().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldop1617().disassembly() != 2i128 {
            return None;
        }
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldop1821()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop0004().disassembly());
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1821 = token_parser.TokenFieldop1821();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:98:1, end:98:2))"]
#[derive(Clone, Debug)]
struct cmpf_d_instructionVar132 {
    fcond2730: TokenField_fcond2730,
    R1115x2: TokenField_R1115x2,
    R0004x2: TokenField_R0004x2,
    fcbit1719: TokenField_fcbit1719,
}
impl cmpf_d_instructionVar132 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 12usize] = [
            DisplayElement::Literal("cmpf.d"),
            DisplayElement::Literal(" "),
            self.fcond2730.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        let R0004x2 = token_parser.TokenFieldR0004x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop3131().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop2126().disassembly() != 33i128 {
            return None;
        }
        if token_parser.TokenFieldop2020().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let fcond2730 = token_parser.TokenFieldfcond2730();
        let fcbit1719 = token_parser.TokenFieldfcbit1719();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:115:1, end:115:2))"]
#[derive(Clone, Debug)]
struct cmpf_s_instructionVar133 {
    fcond2730: TokenField_fcond2730,
    fcbit1719: TokenField_fcbit1719,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl cmpf_s_instructionVar133 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmpf.s"),
            DisplayElement::Literal(" "),
            self.fcond2730.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop3131().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop2126().disassembly() != 33i128 {
            return None;
        }
        if token_parser.TokenFieldop2020().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let fcond2730 = token_parser.TokenFieldfcond2730();
        let fcbit1719 = token_parser.TokenFieldfcbit1719();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:83:1, end:83:2))"]
#[derive(Clone, Debug)]
struct cmovf_d_instructionVar134 {
    fcbit1719: TokenField_fcbit1719,
    R1115x2: TokenField_R1115x2,
    R0004x2: TokenField_R0004x2,
    R2731x2: TokenField_R2731x2,
}
impl cmovf_d_instructionVar134 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 12usize] = [
            DisplayElement::Literal("cmovf.d"),
            DisplayElement::Literal(" "),
            self.fcbit1719.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R1115x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R0004x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115x2 = token_parser.TokenFieldR1115x2();
        let R0004x2 = token_parser.TokenFieldR0004x2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 32i128 {
            return None;
        }
        if token_parser.TokenFieldop2020().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R2731x2 = token_parser.TokenFieldR2731x2();
        let fcbit1719 = token_parser.TokenFieldfcbit1719();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:91:1, end:91:2))"]
#[derive(Clone, Debug)]
struct cmovf_s_instructionVar135 {
    fcbit1719: TokenField_fcbit1719,
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl cmovf_s_instructionVar135 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmovf.s"),
            DisplayElement::Literal(" "),
            self.fcbit1719.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 32i128 {
            return None;
        }
        if token_parser.TokenFieldop2020().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let fcbit1719 = token_parser.TokenFieldfcbit1719();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:75:1, end:75:2))"]
#[derive(Clone, Debug)]
struct mac_instructionVar136 {
    R2731x2: TokenField_R2731x2,
    R1620x2: TokenField_R1620x2,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl mac_instructionVar136 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R2731x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 30i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R1620x2 = token_parser.TokenFieldR1620x2();
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:81:1, end:81:2))"]
#[derive(Clone, Debug)]
struct macu_instructionVar137 {
    R2731x2: TokenField_R2731x2,
    R1620x2: TokenField_R1620x2,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl macu_instructionVar137 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.R2731x2.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 31i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R1620x2 = token_parser.TokenFieldR1620x2();
        let R2731x2 = token_parser.TokenFieldR2731x2();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:133:1, end:133:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar138 {
    s0004: TokenField_s0004,
    R1115: TableR1115,
}
impl mov_instructionVar138 {
    fn display_extend<T>(
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
            self.s0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 16i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.TokenFields0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:139:1, end:139:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar139 {
    op3247: TokenField_op3247,
    op1631: TokenField_op1631,
    R0004: TableR0004,
}
impl mov_instructionVar139 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_imm32: i128 = 0;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .map(|shl| self.op3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op1631.disassembly());
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_imm32),
            DisplayElement::Literal(","),
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
        let mut calc_imm32: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0515().disassembly() != 49i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let op1631 = token_parser.TokenFieldop1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldop3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop1631().disassembly());
        let op3247 = token_parser.TokenFieldop3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:146:1, end:146:2))"]
#[derive(Clone, Debug)]
struct movea_instructionVar140 {
    s1631: TokenField_s1631,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl movea_instructionVar140 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movea"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 49i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:152:1, end:152:2))"]
#[derive(Clone, Debug)]
struct movhi_instructionVar141 {
    s1631: TokenField_s1631,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl movhi_instructionVar141 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movhi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 50i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:181:1, end:181:2))"]
#[derive(Clone, Debug)]
struct satadd_instructionVar142 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl satadd_instructionVar142 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:190:1, end:190:2))"]
#[derive(Clone, Debug)]
struct satadd_instructionVar143 {
    s0004: TokenField_s0004,
    R1115: TableR1115,
}
impl satadd_instructionVar143 {
    fn display_extend<T>(
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
            DisplayElement::Literal("satadd"),
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 17i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.TokenFields0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:208:1, end:208:2))"]
#[derive(Clone, Debug)]
struct satsub_instructionVar144 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl satsub_instructionVar144 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:226:1, end:226:2))"]
#[derive(Clone, Debug)]
struct satsubi_instructionVar145 {
    s1631: TokenField_s1631,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl satsubi_instructionVar145 {
    fn display_extend<T>(
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
            DisplayElement::Literal("satsubi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 51i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:235:1, end:235:2))"]
#[derive(Clone, Debug)]
struct satsubr_instructionVar146 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl satsubr_instructionVar146 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldop1115().disassembly() == 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:247:1, end:247:2))"]
#[derive(Clone, Debug)]
struct cmov_instructionVar147 {
    R0004: TableR0004,
    R1115: TableR1115,
    c1720: Tablec1720,
    R2731: TableR2731,
}
impl cmov_instructionVar147 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 25i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:254:1, end:254:2))"]
#[derive(Clone, Debug)]
struct cmov_instructionVar148 {
    s0004: TokenField_s0004,
    R1115: TableR1115,
    c1720: Tablec1720,
    R2731: TableR2731,
}
impl cmov_instructionVar148 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.TokenFields0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop2126().disassembly() != 24i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct adf_instructionVar149 {
    R0004: TableR0004,
    R1115: TableR1115,
    c1720: Tablec1720,
    R2731: TableR2731,
}
impl adf_instructionVar149 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 29i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:24:1, end:24:2))"]
#[derive(Clone, Debug)]
struct sbf_instructionVar150 {
    R0004: TableR0004,
    R1115: TableR1115,
    c1720: Tablec1720,
    R2731: TableR2731,
}
impl sbf_instructionVar150 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2126().disassembly() != 28i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:322:1, end:322:2))"]
#[derive(Clone, Debug)]
struct maddf_s_instructionVar151 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
    reg4: Tablereg4,
}
impl maddf_s_instructionVar151 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2426().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldop2122().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:356:1, end:356:2))"]
#[derive(Clone, Debug)]
struct msubf_s_instructionVar152 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
    reg4: Tablereg4,
}
impl msubf_s_instructionVar152 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2426().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldop2122().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:386:1, end:386:2))"]
#[derive(Clone, Debug)]
struct nmaddf_s_instructionVar153 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
    reg4: Tablereg4,
}
impl nmaddf_s_instructionVar153 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2426().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldop2122().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:392:1, end:392:2))"]
#[derive(Clone, Debug)]
struct nmsubf_s_instructionVar154 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
    reg4: Tablereg4,
}
impl nmsubf_s_instructionVar154 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R2731.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop2426().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldop2122().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u32);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:44:1, end:44:2))"]
#[derive(Clone, Debug)]
struct ld_h_instructionVar155 {
    s1631: TokenField_s1631,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_h_instructionVar155 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 57i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:59:1, end:59:2))"]
#[derive(Clone, Debug)]
struct ld_hu_instructionVar156 {
    s1731: TokenField_s1731,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_hu_instructionVar156 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = u32::try_from(1i128)
            .ok()
            .map(|shl| self.s1731.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.hu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp16),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0510().disassembly() != 63i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1616().disassembly() != 1i128 {
            return None;
        }
        calc_disp16 = u32::try_from(1i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields1731()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0);
        let s1731 = token_parser.TokenFields1731();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:75:1, end:75:2))"]
#[derive(Clone, Debug)]
struct ld_w_instructionVar157 {
    s1731: TokenField_s1731,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_w_instructionVar157 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = self.s1731.disassembly().wrapping_mul(2i128);
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp16),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0510().disassembly() != 57i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1616().disassembly() != 1i128 {
            return None;
        }
        calc_disp16 = token_parser
            .TokenFields1731()
            .disassembly()
            .wrapping_mul(2i128);
        let s1731 = token_parser.TokenFields1731();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:98:1, end:98:2))"]
#[derive(Clone, Debug)]
struct sld_bu_instructionVar158 {
    op0003: TokenField_op0003,
    R1115: TableR1115,
}
impl sld_bu_instructionVar158 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sld.bu"),
            DisplayElement::Literal(" "),
            self.op0003.display(),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("],"),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0410().disassembly() != 6i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0003 = token_parser.TokenFieldop0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:113:1, end:113:2))"]
#[derive(Clone, Debug)]
struct sld_hu_instructionVar159 {
    op0003: TokenField_op0003,
    R1115: TableR1115,
}
impl sld_hu_instructionVar159 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp5: i128 = 0;
        calc_disp5 = self.op0003.disassembly().wrapping_mul(2i128);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("sld.hu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp5),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("],"),
            DisplayElement::Literal(" "),
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
        let mut calc_disp5: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0410().disassembly() != 7i128 {
            return None;
        }
        calc_disp5 = token_parser
            .TokenFieldop0003()
            .disassembly()
            .wrapping_mul(2i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0003 = token_parser.TokenFieldop0003();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:179:1, end:179:2))"]
#[derive(Clone, Debug)]
struct st_h_instructionVar160 {
    s1631: TokenField_s1631,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl st_h_instructionVar160 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 59i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:196:1, end:196:2))"]
#[derive(Clone, Debug)]
struct st_w_instructionVar161 {
    s1731: TokenField_s1731,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl st_w_instructionVar161 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = self.s1731.disassembly().wrapping_mul(2i128);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("st.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp16),
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
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0510().disassembly() != 59i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        if token_parser.TokenFieldop1616().disassembly() != 1i128 {
            return None;
        }
        calc_disp16 = token_parser
            .TokenFields1731()
            .disassembly()
            .wrapping_mul(2i128);
        let s1731 = token_parser.TokenFields1731();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:347:1, end:347:2))"]
#[derive(Clone, Debug)]
struct sxb_instructionVar162 {
    R0004: TableR0004,
}
impl sxb_instructionVar162 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 5i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:353:1, end:353:2))"]
#[derive(Clone, Debug)]
struct sxh_instructionVar163 {
    R0004: TableR0004,
}
impl sxh_instructionVar163 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 7i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:359:1, end:359:2))"]
#[derive(Clone, Debug)]
struct zxb_instructionVar164 {
    R0004: TableR0004,
}
impl zxb_instructionVar164 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 4i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:365:1, end:365:2))"]
#[derive(Clone, Debug)]
struct zxh_instructionVar165 {
    R0004: TableR0004,
}
impl zxh_instructionVar165 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 6i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:70:1, end:70:2))"]
#[derive(Clone, Debug)]
struct jarl_instructionVar166 {
    R0004: TableR0004,
    adr32: Tableadr32,
}
impl jarl_instructionVar166 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0515().disassembly() != 23i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
            Tableadr32::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:87:1, end:87:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar167 {
    R0004: TableR0004,
    adr32i: Tableadr32i,
}
impl jmp_instructionVar167 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 55i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let adr32i = if let Some((len, table)) = Tableadr32i::parse(
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
        Some((pattern_len, Self { R0004, adr32i }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:318:1, end:318:2))"]
#[derive(Clone, Debug)]
struct switch_instructionVar168 {
    R0004: TableR0004,
}
impl switch_instructionVar168 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0515().disassembly() != 2i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:104:1, end:104:2))"]
#[derive(Clone, Debug)]
struct clr1_instructionVar169 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: TableR0004,
}
impl clr1_instructionVar169 {
    fn display_extend<T>(
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
            DisplayElement::Literal("clr1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 62i128 {
            return None;
        }
        if token_parser.TokenFieldop1415().disassembly() != 2i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.TokenFieldop1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:121:1, end:121:2))"]
#[derive(Clone, Debug)]
struct not1_instructionVar170 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: TableR0004,
}
impl not1_instructionVar170 {
    fn display_extend<T>(
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
            DisplayElement::Literal("not1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 62i128 {
            return None;
        }
        if token_parser.TokenFieldop1415().disassembly() != 1i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.TokenFieldop1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:138:1, end:138:2))"]
#[derive(Clone, Debug)]
struct set1_instructionVar171 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: TableR0004,
}
impl set1_instructionVar171 {
    fn display_extend<T>(
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
            DisplayElement::Literal("set1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 62i128 {
            return None;
        }
        if token_parser.TokenFieldop1415().disassembly() != 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.TokenFieldop1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:155:1, end:155:2))"]
#[derive(Clone, Debug)]
struct tst1_instructionVar172 {
    op1113: TokenField_op1113,
    s1631: TokenField_s1631,
    R0004: TableR0004,
}
impl tst1_instructionVar172 {
    fn display_extend<T>(
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
            DisplayElement::Literal("tst1"),
            DisplayElement::Literal(" "),
            self.op1113.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 62i128 {
            return None;
        }
        if token_parser.TokenFieldop1415().disassembly() != 3i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op1113 = token_parser.TokenFieldop1113();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:39:1, end:39:2))"]
#[derive(Clone, Debug)]
struct mulh_instructionVar173 {
    s0004: TokenField_s0004,
    R1115: TableR1115,
}
impl mulh_instructionVar173 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mulh"),
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 23i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.TokenFields0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:45:1, end:45:2))"]
#[derive(Clone, Debug)]
struct mulhi_instructionVar174 {
    s1631: TokenField_s1631,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl mulhi_instructionVar174 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mulhi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 55i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:94:1, end:94:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar175 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl add_instructionVar175 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 14i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:101:1, end:101:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar176 {
    s0004: TokenField_s0004,
    R1115: TableR1115,
}
impl add_instructionVar176 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 18i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.TokenFields0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:108:1, end:108:2))"]
#[derive(Clone, Debug)]
struct addi_instructionVar177 {
    s1631: TokenField_s1631,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl addi_instructionVar177 {
    fn display_extend<T>(
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
            DisplayElement::Literal("addi"),
            DisplayElement::Literal(" "),
            self.s1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 48i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:115:1, end:115:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar178 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl cmp_instructionVar178 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 15i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:121:1, end:121:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar179 {
    s0004: TokenField_s0004,
    R1115: TableR1115,
}
impl cmp_instructionVar179 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal(" "),
            self.s0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 19i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let s0004 = token_parser.TokenFields0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:127:1, end:127:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar180 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl mov_instructionVar180 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 0i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:158:1, end:158:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar181 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl sub_instructionVar181 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 13i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:165:1, end:165:2))"]
#[derive(Clone, Debug)]
struct subr_instructionVar182 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl subr_instructionVar182 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 12i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:263:1, end:263:2))"]
#[derive(Clone, Debug)]
struct divh_instructionVar183 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl divh_instructionVar183 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 2i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct ld_b_instructionVar184 {
    s1631: TokenField_s1631,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_b_instructionVar184 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 56i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:149:1, end:149:2))"]
#[derive(Clone, Debug)]
struct dispose_instructionVar185 {
    prep0105: TokenField_prep0105,
    DispList: TableDispList,
}
impl dispose_instructionVar185 {
    fn display_extend<T>(
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
            DisplayElement::Literal("dispose"),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldprep0615().disassembly() != 25i128 {
            return None;
        }
        if token_parser.TokenFieldprep1620().disassembly() != 0i128 {
            return None;
        }
        let DispList = if let Some((len, table)) = TableDispList::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.TokenFieldprep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList, prep0105 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar186 {
    prep0105: TokenField_prep0105,
    PrepList: TablePrepList,
}
impl prepare_instructionVar186 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
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
        if token_parser.TokenFieldprep0615().disassembly() != 30i128 {
            return None;
        }
        if token_parser.TokenFieldprep1620().disassembly() != 1i128 {
            return None;
        }
        let PrepList = if let Some((len, table)) = TablePrepList::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.TokenFieldprep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList, prep0105 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:229:1, end:229:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar187 {
    prep0105: TokenField_prep0105,
    PrepList: TablePrepList,
}
impl prepare_instructionVar187 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldprep0615().disassembly() != 30i128 {
            return None;
        }
        if token_parser.TokenFieldprep1620().disassembly() != 3i128 {
            return None;
        }
        let PrepList = if let Some((len, table)) = TablePrepList::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.TokenFieldprep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList, prep0105 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:237:1, end:237:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar188 {
    prep0105: TokenField_prep0105,
    s3247: TokenField_s3247,
    PrepList: TablePrepList,
}
impl prepare_instructionVar188 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldprep0615().disassembly() != 30i128 {
            return None;
        }
        if token_parser.TokenFieldprep1620().disassembly() != 11i128 {
            return None;
        }
        let PrepList = if let Some((len, table)) = TablePrepList::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.TokenFieldprep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:245:1, end:245:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar189 {
    prep0105: TokenField_prep0105,
    s3247: TokenField_s3247,
    PrepList: TablePrepList,
}
impl prepare_instructionVar189 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldprep0615().disassembly() != 30i128 {
            return None;
        }
        if token_parser.TokenFieldprep1620().disassembly() != 19i128 {
            return None;
        }
        let PrepList = if let Some((len, table)) = TablePrepList::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.TokenFieldprep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s3247 = token_parser.TokenFields3247();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:253:1, end:253:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar190 {
    prep0105: TokenField_prep0105,
    op4863: TokenField_op4863,
    op3247: TokenField_op3247,
    PrepList: TablePrepList,
}
impl prepare_instructionVar190 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_imm32: i128 = 0;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .map(|shl| self.op4863.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op3247.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("prepare"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_imm32),
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
        let mut calc_imm32: i128 = 0;
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldprep0615().disassembly() != 30i128 {
            return None;
        }
        if token_parser.TokenFieldprep1620().disassembly() != 27i128 {
            return None;
        }
        let PrepList = if let Some((len, table)) = TablePrepList::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep0105 = token_parser.TokenFieldprep0105();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op3247 = token_parser.TokenFieldop3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldop4863()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop3247().disassembly());
        let op4863 = token_parser.TokenFieldop4863();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:28:1, end:28:2))"]
#[derive(Clone, Debug)]
struct ld_bu_instructionVar191 {
    s1731: TokenField_s1731,
    op0505: TokenField_op0505,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_bu_instructionVar191 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = (u32::try_from(1i128)
            .ok()
            .map(|shl| self.s1731.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op0505.disassembly());
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ld.bu"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp16),
            DisplayElement::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("],"), DisplayElement::Literal(" ")];
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
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0610().disassembly() != 30i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0505 = token_parser.TokenFieldop0505();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1616().disassembly() != 1i128 {
            return None;
        }
        calc_disp16 = (u32::try_from(1i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields1731()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop0505().disassembly());
        let s1731 = token_parser.TokenFields1731();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:57:1, end:57:2))"]
#[derive(Clone, Debug)]
struct br_instructionVar192 {
    adr9: Tableadr9,
}
impl br_instructionVar192 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0710().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldop0003().disassembly() != 5i128 {
            return None;
        }
        let adr9 = if let Some((len, table)) =
            Tableadr9::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:121:1, end:121:2))"]
#[derive(Clone, Debug)]
struct sld_w_instructionVar193 {
    op0106: TokenField_op0106,
    R1115: TableR1115,
}
impl sld_w_instructionVar193 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = self.op0106.disassembly().wrapping_mul(4i128);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("sld.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp8),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("],"),
            DisplayElement::Literal(" "),
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
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0710().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0000().disassembly() != 0i128 {
            return None;
        }
        calc_disp8 = token_parser
            .TokenFieldop0106()
            .disassembly()
            .wrapping_mul(4i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0106 = token_parser.TokenFieldop0106();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0106 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:153:1, end:153:2))"]
#[derive(Clone, Debug)]
struct sst_w_instructionVar194 {
    op0106: TokenField_op0106,
    R1115: TableR1115,
}
impl sst_w_instructionVar194 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = self.op0106.disassembly().wrapping_mul(4i128);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sst.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp8),
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
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0710().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldop0000().disassembly() != 1i128 {
            return None;
        }
        calc_disp8 = token_parser
            .TokenFieldop0106()
            .disassembly()
            .wrapping_mul(4i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0106 = token_parser.TokenFieldop0106();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0106 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct st_b_instructionVar195 {
    s1631: TokenField_s1631,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl st_b_instructionVar195 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 58i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
        let s1631 = token_parser.TokenFields1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:287:1, end:287:2))"]
#[derive(Clone, Debug)]
struct sar_instructionVar196 {
    op0004: TokenField_op0004,
    R1115: TableR1115,
}
impl sar_instructionVar196 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sar"),
            DisplayElement::Literal(" "),
            self.op0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 21i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.TokenFieldop0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:317:1, end:317:2))"]
#[derive(Clone, Debug)]
struct shl_instructionVar197 {
    op0004: TokenField_op0004,
    R1115: TableR1115,
}
impl shl_instructionVar197 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shl"),
            DisplayElement::Literal(" "),
            self.op0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 22i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.TokenFieldop0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:335:1, end:335:2))"]
#[derive(Clone, Debug)]
struct shr_instructionVar198 {
    op0004: TokenField_op0004,
    R1115: TableR1115,
}
impl shr_instructionVar198 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shr"),
            DisplayElement::Literal(" "),
            self.op0004.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0510().disassembly() != 20i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0004 = token_parser.TokenFieldop0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:42:1, end:42:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar199 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl and_instructionVar199 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 10i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:49:1, end:49:2))"]
#[derive(Clone, Debug)]
struct andi_instructionVar200 {
    op1631: TokenField_op1631,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl andi_instructionVar200 {
    fn display_extend<T>(
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
            DisplayElement::Literal("andi"),
            DisplayElement::Literal(" "),
            self.op1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 54i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let op1631 = token_parser.TokenFieldop1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:56:1, end:56:2))"]
#[derive(Clone, Debug)]
struct not_instructionVar201 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl not_instructionVar201 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 1i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:63:1, end:63:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar202 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl or_instructionVar202 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 8i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:70:1, end:70:2))"]
#[derive(Clone, Debug)]
struct ori_instructionVar203 {
    op1631: TokenField_op1631,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl ori_instructionVar203 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ori"),
            DisplayElement::Literal(" "),
            self.op1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 52i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let op1631 = token_parser.TokenFieldop1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:77:1, end:77:2))"]
#[derive(Clone, Debug)]
struct tst_instructionVar204 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl tst_instructionVar204 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 11i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:83:1, end:83:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar205 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl xor_instructionVar205 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 9i128 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:90:1, end:90:2))"]
#[derive(Clone, Debug)]
struct xori_instructionVar206 {
    op1631: TokenField_op1631,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl xori_instructionVar206 {
    fn display_extend<T>(
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
            DisplayElement::Literal("xori"),
            DisplayElement::Literal(" "),
            self.op1631.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop0510().disassembly() != 53i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
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
        let op1631 = token_parser.TokenFieldop1631();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:94:1, end:94:2))"]
#[derive(Clone, Debug)]
struct jr_instructionVar207 {
    adr22: Tableadr22,
}
impl jr_instructionVar207 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0615().disassembly() != 30i128 {
            return None;
        }
        let adr22 = if let Some((len, table)) =
            Tableadr22::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:113:1, end:113:2))"]
#[derive(Clone, Debug)]
struct callt_instructionVar208 {
    op0005: TokenField_op0005,
}
impl callt_instructionVar208 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0615().disassembly() != 8i128 {
            return None;
        }
        let op0005 = token_parser.TokenFieldop0005();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0005 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:156:1, end:156:2))"]
#[derive(Clone, Debug)]
struct dispose_instructionVar209 {
    prep0105: TokenField_prep0105,
    prep1620: TokenField_prep1620,
    DispList: TableDispList,
}
impl dispose_instructionVar209 {
    fn display_extend<T>(
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
            DisplayElement::Literal("dispose"),
            DisplayElement::Literal(" "),
            self.prep0105.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.DispList.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("["),
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
        if token_parser.TokenFieldprep0615().disassembly() != 25i128 {
            return None;
        }
        let DispList = if let Some((len, table)) = TableDispList::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let prep1620 = token_parser.TokenFieldprep1620();
        let prep0105 = token_parser.TokenFieldprep0105();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:63:1, end:63:2))"]
#[derive(Clone, Debug)]
struct jarl_instructionVar210 {
    R1115: TableR1115,
    adr22: Tableadr22,
}
impl jarl_instructionVar210 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u32;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u32;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop0610().disassembly() != 30i128 {
                return None;
            }
            let R1115 = if let Some((len, table)) =
                TableR1115::parse(tokens, &mut context_instance, inst_start)
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
            sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let adr22 = if let Some((len, table)) =
            Tableadr22::parse(tokens_current, &mut context_instance, inst_start)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:91:1, end:91:2))"]
#[derive(Clone, Debug)]
struct sld_b_instructionVar211 {
    op0006: TokenField_op0006,
    R1115: TableR1115,
}
impl sld_b_instructionVar211 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sld.b"),
            DisplayElement::Literal(" "),
            self.op0006.display(),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("],"),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0710().disassembly() != 6i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.TokenFieldop0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:105:1, end:105:2))"]
#[derive(Clone, Debug)]
struct sld_h_instructionVar212 {
    op0006: TokenField_op0006,
    R1115: TableR1115,
}
impl sld_h_instructionVar212 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = self.op0006.disassembly().wrapping_mul(2i128);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("sld.h"),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp8),
            DisplayElement::Literal("["),
            DisplayElement::Register(Register::ep),
            DisplayElement::Literal("],"),
            DisplayElement::Literal(" "),
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
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0710().disassembly() != 8i128 {
            return None;
        }
        calc_disp8 = token_parser
            .TokenFieldop0006()
            .disassembly()
            .wrapping_mul(2i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.TokenFieldop0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:136:1, end:136:2))"]
#[derive(Clone, Debug)]
struct sst_b_instructionVar213 {
    op0006: TokenField_op0006,
    R1115: TableR1115,
}
impl sst_b_instructionVar213 {
    fn display_extend<T>(
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldop0710().disassembly() != 7i128 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.TokenFieldop0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:144:1, end:144:2))"]
#[derive(Clone, Debug)]
struct sst_h_instructionVar214 {
    op0006: TokenField_op0006,
    R1115: TableR1115,
}
impl sst_h_instructionVar214 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = self.op0006.disassembly().wrapping_mul(2i128);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sst.h"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Number(true, calc_disp8),
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
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop0710().disassembly() != 9i128 {
            return None;
        }
        calc_disp8 = token_parser
            .TokenFieldop0006()
            .disassembly()
            .wrapping_mul(2i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let op0006 = token_parser.TokenFieldop0006();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:52:1, end:52:2))"]
#[derive(Clone, Debug)]
struct b_instructionVar215 {
    c0003: Tablec0003,
    adr9: Tableadr9,
}
impl b_instructionVar215 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldop0710().disassembly() != 11i128 {
            return None;
        }
        let c0003 = if let Some((len, table)) =
            Tablec0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let adr9 = if let Some((len, table)) =
            Tableadr9::parse(tokens_current, &mut context_instance, inst_start)
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
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(di_instructionVar0),
    Var1(ei_instructionVar1),
    Var2(halt_instructionVar2),
    Var3(absf_d_instructionVar3),
    Var4(absf_s_instructionVar4),
    Var5(ceilf_dl_instructionVar5),
    Var6(ceilf_dul_instructionVar6),
    Var7(ceilf_duw_instructionVar7),
    Var8(ceilf_dw_instructionVar8),
    Var9(ceilf_sl_instructionVar9),
    Var10(ceilf_sul_instructionVar10),
    Var11(ceilf_sul_instructionVar11),
    Var12(ceilf_sw_instructionVar12),
    Var13(cvtf_dl_instructionVar13),
    Var14(cvtf_ds_instructionVar14),
    Var15(cvtf_dul_instructionVar15),
    Var16(cvtf_duw_instructionVar16),
    Var17(cvtf_sw_instructionVar17),
    Var18(cvtf_ls_instructionVar18),
    Var19(cvtf_ls_instructionVar19),
    Var20(cvtf_sd_instructionVar20),
    Var21(cvtf_sl_instructionVar21),
    Var22(cvtf_sul_instructionVar22),
    Var23(cvtf_suw_instructionVar23),
    Var24(cvtf_sw_instructionVar24),
    Var25(cvtf_uls_instructionVar25),
    Var26(cvtf_uls_instructionVar26),
    Var27(cvtf_uwd_instructionVar27),
    Var28(cvtf_uws_instructionVar28),
    Var29(cvtf_wd_instructionVar29),
    Var30(cvtf_ws_instructionVar30),
    Var31(floorf_dl_instructionVar31),
    Var32(floorf_dul_instructionVar32),
    Var33(floorf_duw_instructionVar33),
    Var34(floorf_dw_instructionVar34),
    Var35(floorf_sl_instructionVar35),
    Var36(floorf_sul_instructionVar36),
    Var37(floorf_suw_instructionVar37),
    Var38(floorf_suw_instructionVar38),
    Var39(negf_d_instructionVar39),
    Var40(negf_s_instructionVar40),
    Var41(recipf_d_instructionVar41),
    Var42(recipf_s_instructionVar42),
    Var43(rsqrtf_d_instructionVar43),
    Var44(rsqrtf_s_instructionVar44),
    Var45(sqrtf_d_instructionVar45),
    Var46(sqrtf_s_instructionVar46),
    Var47(trncf_dl_instructionVar47),
    Var48(trncf_dul_instructionVar48),
    Var49(trncf_duw_instructionVar49),
    Var50(trncf_dw_instructionVar50),
    Var51(trncf_sl_instructionVar51),
    Var52(trncf_sul_instructionVar52),
    Var53(trncf_suw_instructionVar53),
    Var54(trncf_sw_instructionVar54),
    Var55(bsh_instructionVar55),
    Var56(bsw_instructionVar56),
    Var57(hsh_instructionVar57),
    Var58(hsw_instructionVar58),
    Var59(sasf_instructionVar59),
    Var60(setf_instructionVar60),
    Var61(ctret_instructionVar61),
    Var62(eiret_instructionVar62),
    Var63(feret_instructionVar63),
    Var64(reti_instructionVar64),
    Var65(rie_instructionVar65),
    Var66(trap_instructionVar66),
    Var67(sar_instructionVar67),
    Var68(shl_instructionVar68),
    Var69(shr_instructionVar69),
    Var70(clr1_instructionVar70),
    Var71(not1_instructionVar71),
    Var72(set1_instructionVar72),
    Var73(tst1_instructionVar73),
    Var74(sch0l_instructionVar74),
    Var75(sch0r_instructionVar75),
    Var76(sch1l_instructionVar76),
    Var77(sch1r_instructionVar77),
    Var78(ldsr_instructionVar78),
    Var79(stsr_instructionVar79),
    Var80(syscall_instructionVar80),
    Var81(mul_instructionVar81),
    Var82(mulu_instructionVar82),
    Var83(satadd_instructionVar83),
    Var84(satsub_instructionVar84),
    Var85(div_instructionVar85),
    Var86(divh_instructionVar86),
    Var87(divhu_instructionVar87),
    Var88(divu_instructionVar88),
    Var89(divq_instructionVar89),
    Var90(divqu_instructionVar90),
    Var91(addf_d_instructionVar91),
    Var92(addf_s_instructionVar92),
    Var93(divf_s_instructionVar93),
    Var94(divf_s_instructionVar94),
    Var95(fmaf_s_instructionVar95),
    Var96(fmsf_s_instructionVar96),
    Var97(fnmaf_s_instructionVar97),
    Var98(fnmfs_s_instructionVar98),
    Var99(maxf_d_instructionVar99),
    Var100(maxf_s_instructionVar100),
    Var101(minf_d_instructionVar101),
    Var102(minf_s_instructionVar102),
    Var103(mulf_d_instructionVar103),
    Var104(mulf_s_instructionVar104),
    Var105(subf_d_instructionVar105),
    Var106(subf_s_instructionVar106),
    Var107(sar_instructionVar107),
    Var108(shl_instructionVar108),
    Var109(shr_instructionVar109),
    Var110(caxi_instructionVar110),
    Var111(mul_instructionVar111),
    Var112(trfsr_instructionVar112),
    Var113(ld_h_instructionVar113),
    Var114(ld_w_instructionVar114),
    Var115(st_h_instructionVar115),
    Var116(st_w_instructionVar116),
    Var117(ld_b_instructionVar117),
    Var118(ld_bu_instructionVar118),
    Var119(ld_hu_instructionVar119),
    Var120(st_b_instructionVar120),
    Var121(jmp_instructionVar121),
    Var122(jmp_instructionVar122),
    Var123(jr_instructionVar123),
    Var124(fetrap_instructionVar124),
    Var125(nop_instructionVar125),
    Var126(rie_instructionVar126),
    Var127(synce_instructionVar127),
    Var128(syncm_instructionVar128),
    Var129(syncp_instructionVar129),
    Var130(mulh_instructionVar130),
    Var131(mulu_instructionVar131),
    Var132(cmpf_d_instructionVar132),
    Var133(cmpf_s_instructionVar133),
    Var134(cmovf_d_instructionVar134),
    Var135(cmovf_s_instructionVar135),
    Var136(mac_instructionVar136),
    Var137(macu_instructionVar137),
    Var138(mov_instructionVar138),
    Var139(mov_instructionVar139),
    Var140(movea_instructionVar140),
    Var141(movhi_instructionVar141),
    Var142(satadd_instructionVar142),
    Var143(satadd_instructionVar143),
    Var144(satsub_instructionVar144),
    Var145(satsubi_instructionVar145),
    Var146(satsubr_instructionVar146),
    Var147(cmov_instructionVar147),
    Var148(cmov_instructionVar148),
    Var149(adf_instructionVar149),
    Var150(sbf_instructionVar150),
    Var151(maddf_s_instructionVar151),
    Var152(msubf_s_instructionVar152),
    Var153(nmaddf_s_instructionVar153),
    Var154(nmsubf_s_instructionVar154),
    Var155(ld_h_instructionVar155),
    Var156(ld_hu_instructionVar156),
    Var157(ld_w_instructionVar157),
    Var158(sld_bu_instructionVar158),
    Var159(sld_hu_instructionVar159),
    Var160(st_h_instructionVar160),
    Var161(st_w_instructionVar161),
    Var162(sxb_instructionVar162),
    Var163(sxh_instructionVar163),
    Var164(zxb_instructionVar164),
    Var165(zxh_instructionVar165),
    Var166(jarl_instructionVar166),
    Var167(jmp_instructionVar167),
    Var168(switch_instructionVar168),
    Var169(clr1_instructionVar169),
    Var170(not1_instructionVar170),
    Var171(set1_instructionVar171),
    Var172(tst1_instructionVar172),
    Var173(mulh_instructionVar173),
    Var174(mulhi_instructionVar174),
    Var175(add_instructionVar175),
    Var176(add_instructionVar176),
    Var177(addi_instructionVar177),
    Var178(cmp_instructionVar178),
    Var179(cmp_instructionVar179),
    Var180(mov_instructionVar180),
    Var181(sub_instructionVar181),
    Var182(subr_instructionVar182),
    Var183(divh_instructionVar183),
    Var184(ld_b_instructionVar184),
    Var185(dispose_instructionVar185),
    Var186(prepare_instructionVar186),
    Var187(prepare_instructionVar187),
    Var188(prepare_instructionVar188),
    Var189(prepare_instructionVar189),
    Var190(prepare_instructionVar190),
    Var191(ld_bu_instructionVar191),
    Var192(br_instructionVar192),
    Var193(sld_w_instructionVar193),
    Var194(sst_w_instructionVar194),
    Var195(st_b_instructionVar195),
    Var196(sar_instructionVar196),
    Var197(shl_instructionVar197),
    Var198(shr_instructionVar198),
    Var199(and_instructionVar199),
    Var200(andi_instructionVar200),
    Var201(not_instructionVar201),
    Var202(or_instructionVar202),
    Var203(ori_instructionVar203),
    Var204(tst_instructionVar204),
    Var205(xor_instructionVar205),
    Var206(xori_instructionVar206),
    Var207(jr_instructionVar207),
    Var208(callt_instructionVar208),
    Var209(dispose_instructionVar209),
    Var210(jarl_instructionVar210),
    Var211(sld_b_instructionVar211),
    Var212(sld_h_instructionVar212),
    Var213(sst_b_instructionVar213),
    Var214(sst_h_instructionVar214),
    Var215(b_instructionVar215),
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
            Self::Var212(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var213(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var214(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var215(x) => x.display_extend(
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
        if let Some((inst_len, parsed)) = di_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = ei_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = halt_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = absf_d_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = absf_s_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_dl_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_dul_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_duw_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_dw_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_sl_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_sul_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_sul_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = ceilf_sw_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_dl_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_ds_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_dul_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_duw_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_sw_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_ls_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_ls_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_sd_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_sl_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_sul_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_suw_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_sw_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_uls_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_uls_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_uwd_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_uws_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_wd_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = cvtf_ws_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_dl_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_dul_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_duw_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_dw_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_sl_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_sul_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_suw_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = floorf_suw_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = negf_d_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = negf_s_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = recipf_d_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = recipf_s_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = rsqrtf_d_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = rsqrtf_s_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = sqrtf_d_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = sqrtf_s_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_dl_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_dul_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_duw_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_dw_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_sl_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_sul_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_suw_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = trncf_sw_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = bsh_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = bsw_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = hsh_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = hsw_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = sasf_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = setf_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = ctret_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = eiret_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = feret_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = reti_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = rie_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = trap_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = sar_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = shl_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = shr_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = clr1_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = not1_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = set1_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = tst1_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = sch0l_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = sch0r_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = sch1l_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = sch1r_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = ldsr_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = stsr_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = syscall_instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = mul_instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = mulu_instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = satadd_instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = satsub_instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = div_instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = divh_instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = divhu_instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = divu_instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = divq_instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = divqu_instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = addf_d_instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = addf_s_instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = divf_s_instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var93(parsed)));
        }
        if let Some((inst_len, parsed)) = divf_s_instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var94(parsed)));
        }
        if let Some((inst_len, parsed)) = fmaf_s_instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var95(parsed)));
        }
        if let Some((inst_len, parsed)) = fmsf_s_instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var96(parsed)));
        }
        if let Some((inst_len, parsed)) = fnmaf_s_instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var97(parsed)));
        }
        if let Some((inst_len, parsed)) = fnmfs_s_instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = maxf_d_instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = maxf_s_instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = minf_d_instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = minf_s_instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = mulf_d_instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = mulf_s_instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = subf_d_instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = subf_s_instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = sar_instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = shl_instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = shr_instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        if let Some((inst_len, parsed)) = caxi_instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = mul_instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = trfsr_instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_h_instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_w_instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = st_h_instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = st_w_instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_b_instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_bu_instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_hu_instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = st_b_instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = jmp_instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = jmp_instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var122(parsed)));
        }
        if let Some((inst_len, parsed)) = jr_instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var123(parsed)));
        }
        if let Some((inst_len, parsed)) = fetrap_instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var124(parsed)));
        }
        if let Some((inst_len, parsed)) = nop_instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var125(parsed)));
        }
        if let Some((inst_len, parsed)) = rie_instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var126(parsed)));
        }
        if let Some((inst_len, parsed)) = synce_instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var127(parsed)));
        }
        if let Some((inst_len, parsed)) = syncm_instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var128(parsed)));
        }
        if let Some((inst_len, parsed)) = syncp_instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var129(parsed)));
        }
        if let Some((inst_len, parsed)) = mulh_instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var130(parsed)));
        }
        if let Some((inst_len, parsed)) = mulu_instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var131(parsed)));
        }
        if let Some((inst_len, parsed)) = cmpf_d_instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var132(parsed)));
        }
        if let Some((inst_len, parsed)) = cmpf_s_instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var133(parsed)));
        }
        if let Some((inst_len, parsed)) = cmovf_d_instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var134(parsed)));
        }
        if let Some((inst_len, parsed)) = cmovf_s_instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var135(parsed)));
        }
        if let Some((inst_len, parsed)) = mac_instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var136(parsed)));
        }
        if let Some((inst_len, parsed)) = macu_instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var137(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var138(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var139(parsed)));
        }
        if let Some((inst_len, parsed)) = movea_instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var140(parsed)));
        }
        if let Some((inst_len, parsed)) = movhi_instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var141(parsed)));
        }
        if let Some((inst_len, parsed)) = satadd_instructionVar142::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var142(parsed)));
        }
        if let Some((inst_len, parsed)) = satadd_instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var143(parsed)));
        }
        if let Some((inst_len, parsed)) = satsub_instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var144(parsed)));
        }
        if let Some((inst_len, parsed)) = satsubi_instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var145(parsed)));
        }
        if let Some((inst_len, parsed)) = satsubr_instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var146(parsed)));
        }
        if let Some((inst_len, parsed)) = cmov_instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var147(parsed)));
        }
        if let Some((inst_len, parsed)) = cmov_instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var148(parsed)));
        }
        if let Some((inst_len, parsed)) = adf_instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var149(parsed)));
        }
        if let Some((inst_len, parsed)) = sbf_instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var150(parsed)));
        }
        if let Some((inst_len, parsed)) = maddf_s_instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var151(parsed)));
        }
        if let Some((inst_len, parsed)) = msubf_s_instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var152(parsed)));
        }
        if let Some((inst_len, parsed)) = nmaddf_s_instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var153(parsed)));
        }
        if let Some((inst_len, parsed)) = nmsubf_s_instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var154(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_h_instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var155(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_hu_instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var156(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_w_instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var157(parsed)));
        }
        if let Some((inst_len, parsed)) = sld_bu_instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var158(parsed)));
        }
        if let Some((inst_len, parsed)) = sld_hu_instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var159(parsed)));
        }
        if let Some((inst_len, parsed)) = st_h_instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var160(parsed)));
        }
        if let Some((inst_len, parsed)) = st_w_instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var161(parsed)));
        }
        if let Some((inst_len, parsed)) = sxb_instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var162(parsed)));
        }
        if let Some((inst_len, parsed)) = sxh_instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var163(parsed)));
        }
        if let Some((inst_len, parsed)) = zxb_instructionVar164::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var164(parsed)));
        }
        if let Some((inst_len, parsed)) = zxh_instructionVar165::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var165(parsed)));
        }
        if let Some((inst_len, parsed)) = jarl_instructionVar166::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var166(parsed)));
        }
        if let Some((inst_len, parsed)) = jmp_instructionVar167::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var167(parsed)));
        }
        if let Some((inst_len, parsed)) = switch_instructionVar168::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var168(parsed)));
        }
        if let Some((inst_len, parsed)) = clr1_instructionVar169::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var169(parsed)));
        }
        if let Some((inst_len, parsed)) = not1_instructionVar170::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var170(parsed)));
        }
        if let Some((inst_len, parsed)) = set1_instructionVar171::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var171(parsed)));
        }
        if let Some((inst_len, parsed)) = tst1_instructionVar172::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var172(parsed)));
        }
        if let Some((inst_len, parsed)) = mulh_instructionVar173::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var173(parsed)));
        }
        if let Some((inst_len, parsed)) = mulhi_instructionVar174::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var174(parsed)));
        }
        if let Some((inst_len, parsed)) = add_instructionVar175::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var175(parsed)));
        }
        if let Some((inst_len, parsed)) = add_instructionVar176::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var176(parsed)));
        }
        if let Some((inst_len, parsed)) = addi_instructionVar177::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var177(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar178::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var178(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar179::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var179(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_instructionVar180::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var180(parsed)));
        }
        if let Some((inst_len, parsed)) = sub_instructionVar181::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var181(parsed)));
        }
        if let Some((inst_len, parsed)) = subr_instructionVar182::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var182(parsed)));
        }
        if let Some((inst_len, parsed)) = divh_instructionVar183::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var183(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_b_instructionVar184::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var184(parsed)));
        }
        if let Some((inst_len, parsed)) = dispose_instructionVar185::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var185(parsed)));
        }
        if let Some((inst_len, parsed)) = prepare_instructionVar186::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var186(parsed)));
        }
        if let Some((inst_len, parsed)) = prepare_instructionVar187::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var187(parsed)));
        }
        if let Some((inst_len, parsed)) = prepare_instructionVar188::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var188(parsed)));
        }
        if let Some((inst_len, parsed)) = prepare_instructionVar189::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var189(parsed)));
        }
        if let Some((inst_len, parsed)) = prepare_instructionVar190::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var190(parsed)));
        }
        if let Some((inst_len, parsed)) = ld_bu_instructionVar191::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var191(parsed)));
        }
        if let Some((inst_len, parsed)) = br_instructionVar192::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var192(parsed)));
        }
        if let Some((inst_len, parsed)) = sld_w_instructionVar193::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var193(parsed)));
        }
        if let Some((inst_len, parsed)) = sst_w_instructionVar194::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var194(parsed)));
        }
        if let Some((inst_len, parsed)) = st_b_instructionVar195::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var195(parsed)));
        }
        if let Some((inst_len, parsed)) = sar_instructionVar196::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var196(parsed)));
        }
        if let Some((inst_len, parsed)) = shl_instructionVar197::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var197(parsed)));
        }
        if let Some((inst_len, parsed)) = shr_instructionVar198::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var198(parsed)));
        }
        if let Some((inst_len, parsed)) = and_instructionVar199::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var199(parsed)));
        }
        if let Some((inst_len, parsed)) = andi_instructionVar200::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var200(parsed)));
        }
        if let Some((inst_len, parsed)) = not_instructionVar201::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var201(parsed)));
        }
        if let Some((inst_len, parsed)) = or_instructionVar202::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var202(parsed)));
        }
        if let Some((inst_len, parsed)) = ori_instructionVar203::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var203(parsed)));
        }
        if let Some((inst_len, parsed)) = tst_instructionVar204::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var204(parsed)));
        }
        if let Some((inst_len, parsed)) = xor_instructionVar205::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var205(parsed)));
        }
        if let Some((inst_len, parsed)) = xori_instructionVar206::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var206(parsed)));
        }
        if let Some((inst_len, parsed)) = jr_instructionVar207::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var207(parsed)));
        }
        if let Some((inst_len, parsed)) = callt_instructionVar208::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var208(parsed)));
        }
        if let Some((inst_len, parsed)) = dispose_instructionVar209::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var209(parsed)));
        }
        if let Some((inst_len, parsed)) = jarl_instructionVar210::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var210(parsed)));
        }
        if let Some((inst_len, parsed)) = sld_b_instructionVar211::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var211(parsed)));
        }
        if let Some((inst_len, parsed)) = sld_h_instructionVar212::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var212(parsed)));
        }
        if let Some((inst_len, parsed)) = sst_b_instructionVar213::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var213(parsed)));
        }
        if let Some((inst_len, parsed)) = sst_h_instructionVar214::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var214(parsed)));
        }
        if let Some((inst_len, parsed)) = b_instructionVar215::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var215(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:6:1, end:6:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:7:1, end:7:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 8i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:8:1, end:8:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:9:1, end:9:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 9i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:10:1, end:10:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:11:1, end:11:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 10i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:12:1, end:12:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:13:1, end:13:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 11i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:14:1, end:14:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:15:1, end:15:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 12i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:16:1, end:16:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:17:1, end:17:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 13i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:18:1, end:18:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:19:1, end:19:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 14i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:20:1, end:20:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:21:1, end:21:6))"]
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
        if token_parser.TokenFieldop0003().disassembly() != 15i128 {
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
enum Tablec0003 {
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
impl Tablec0003 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:23:1, end:23:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:24:1, end:24:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 8i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:25:1, end:25:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:26:1, end:26:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 9i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:27:1, end:27:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:28:1, end:28:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 10i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:29:1, end:29:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:30:1, end:30:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 11i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:31:1, end:31:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:32:1, end:32:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 12i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:33:1, end:33:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:34:1, end:34:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 13i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:35:1, end:35:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:36:1, end:36:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 14i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:37:1, end:37:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:38:1, end:38:6))"]
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
        if token_parser.TokenFieldop1720().disassembly() != 15i128 {
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
enum Tablec1720 {
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
impl Tablec1720 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:7:1, end:7:6))"]
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
        if token_parser.TokenField_R0004().disassembly() != 0i128 {
            return None;
        }
        let _R0004 = token_parser.TokenField_R0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:8:1, end:8:6))"]
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
        let _R0004 = token_parser.TokenField_R0004();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R0004 }))
    }
}
#[derive(Clone, Debug)]
enum TableR0004 {
    Var0(R0004Var0),
    Var1(R0004Var1),
}
impl TableR0004 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:10:1, end:10:6))"]
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
        if token_parser.TokenField_R1115().disassembly() != 0i128 {
            return None;
        }
        let _R1115 = token_parser.TokenField_R1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:11:1, end:11:6))"]
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
        let _R1115 = token_parser.TokenField_R1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R1115 }))
    }
}
#[derive(Clone, Debug)]
enum TableR1115 {
    Var0(R1115Var0),
    Var1(R1115Var1),
}
impl TableR1115 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:13:1, end:13:6))"]
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
        if token_parser.TokenField_R2731().disassembly() != 0i128 {
            return None;
        }
        let _R2731 = token_parser.TokenField_R2731();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:14:1, end:14:6))"]
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
        let _R2731 = token_parser.TokenField_R2731();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R2731 }))
    }
}
#[derive(Clone, Debug)]
enum TableR2731 {
    Var0(R2731Var0),
    Var1(R2731Var1),
}
impl TableR2731 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:17:1, end:17:5))"]
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
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(4i128)
            .ok()
            .map(|shl| self.s1115.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| self.op0406.disassembly().checked_shl(shl))
                .flatten()
                .unwrap_or(0))
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_res)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_res = (u32::try_from(4i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields1115()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .map(|shl| {
                    token_parser
                        .TokenFieldop0406()
                        .disassembly()
                        .checked_shl(shl)
                })
                .flatten()
                .unwrap_or(0))
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let op0406 = token_parser.TokenFieldop0406();
        let s1115 = token_parser.TokenFields1115();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0406, s1115 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr9 {
    Var0(adr9Var0),
}
impl Tableadr9 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:23:1, end:23:6))"]
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
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(16i128)
            .ok()
            .map(|shl| self.s0005.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op1631.disassembly())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_res)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let s0005 = token_parser.TokenFields0005();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        calc_res = (u32::try_from(16i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFields0005()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop1631().disassembly())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let op1631 = token_parser.TokenFieldop1631();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { s0005, op1631 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr22 {
    Var0(adr22Var0),
}
impl Tableadr22 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:29:1, end:29:6))"]
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
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(16i128)
            .ok()
            .map(|shl| self.op3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op1631.disassembly())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_res)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let op1631 = token_parser.TokenFieldop1631();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_res = (u32::try_from(16i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldop3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop1631().disassembly())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let op3247 = token_parser.TokenFieldop3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1631, op3247 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr32 {
    Var0(adr32Var0),
}
impl Tableadr32 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:35:1, end:35:7))"]
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
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(16i128)
            .ok()
            .map(|shl| self.op3247.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            | self.op1631.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_res)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop1616().disassembly() != 0i128 {
            return None;
        }
        let op1631 = token_parser.TokenFieldop1631();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_res = (u32::try_from(16i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldop3247()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
            .unwrap_or(0)
            | token_parser.TokenFieldop1631().disassembly());
        let op3247 = token_parser.TokenFieldop3247();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1631, op3247 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr32i {
    Var0(adr32iVar0),
}
impl Tableadr32i {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:41:1, end:41:5))"]
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
        if token_parser.TokenFieldop2323().disassembly() != 0i128 {
            return None;
        }
        let op0_1720 = token_parser.TokenFieldop0_1720();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0_1720 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:42:1, end:42:5))"]
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
        if token_parser.TokenFieldop2323().disassembly() != 1i128 {
            return None;
        }
        let op1_1720 = token_parser.TokenFieldop1_1720();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1_1720 }))
    }
}
#[derive(Clone, Debug)]
enum Tablereg4 {
    Var0(reg4Var0),
    Var1(reg4Var1),
}
impl Tablereg4 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:48:1, end:48:11))"]
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
        if token_parser.TokenFieldprep27().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:49:1, end:49:11))"]
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
        if token_parser.TokenFieldprep27().disassembly() != 0i128 {
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
enum TablePrepList20 {
    Var0(PrepList20Var0),
    Var1(PrepList20Var1),
}
impl TablePrepList20 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:51:1, end:51:11))"]
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
        if token_parser.TokenFieldprep26().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep27().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:52:1, end:52:11))"]
#[derive(Clone, Debug)]
struct PrepList21Var1 {
    PrepList20: TablePrepList20,
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
        if token_parser.TokenFieldprep26().disassembly() != 1i128 {
            return None;
        }
        let PrepList20 = if let Some((len, table)) = TablePrepList20::parse(
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
        Some((pattern_len, Self { PrepList20 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:53:1, end:53:11))"]
#[derive(Clone, Debug)]
struct PrepList21Var2 {
    PrepList20: TablePrepList20,
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
        if token_parser.TokenFieldprep26().disassembly() != 0i128 {
            return None;
        }
        let PrepList20 = if let Some((len, table)) = TablePrepList20::parse(
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
        Some((pattern_len, Self { PrepList20 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList21 {
    Var0(PrepList21Var0),
    Var1(PrepList21Var1),
    Var2(PrepList21Var2),
}
impl TablePrepList21 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:55:1, end:55:11))"]
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
        if token_parser.TokenFieldprep25().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2627().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:56:1, end:56:11))"]
#[derive(Clone, Debug)]
struct PrepList22Var1 {
    PrepList21: TablePrepList21,
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
        if token_parser.TokenFieldprep25().disassembly() != 1i128 {
            return None;
        }
        let PrepList21 = if let Some((len, table)) = TablePrepList21::parse(
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
        Some((pattern_len, Self { PrepList21 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:57:1, end:57:11))"]
#[derive(Clone, Debug)]
struct PrepList22Var2 {
    PrepList21: TablePrepList21,
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
        if token_parser.TokenFieldprep25().disassembly() != 0i128 {
            return None;
        }
        let PrepList21 = if let Some((len, table)) = TablePrepList21::parse(
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
        Some((pattern_len, Self { PrepList21 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList22 {
    Var0(PrepList22Var0),
    Var1(PrepList22Var1),
    Var2(PrepList22Var2),
}
impl TablePrepList22 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:59:1, end:59:11))"]
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
        if token_parser.TokenFieldprep24().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2527().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:60:1, end:60:11))"]
#[derive(Clone, Debug)]
struct PrepList23Var1 {
    PrepList22: TablePrepList22,
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
        if token_parser.TokenFieldprep24().disassembly() != 1i128 {
            return None;
        }
        let PrepList22 = if let Some((len, table)) = TablePrepList22::parse(
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
        Some((pattern_len, Self { PrepList22 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:61:1, end:61:11))"]
#[derive(Clone, Debug)]
struct PrepList23Var2 {
    PrepList22: TablePrepList22,
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
        if token_parser.TokenFieldprep24().disassembly() != 0i128 {
            return None;
        }
        let PrepList22 = if let Some((len, table)) = TablePrepList22::parse(
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
        Some((pattern_len, Self { PrepList22 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList23 {
    Var0(PrepList23Var0),
    Var1(PrepList23Var1),
    Var2(PrepList23Var2),
}
impl TablePrepList23 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:63:1, end:63:11))"]
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
        if token_parser.TokenFieldprep31().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2427().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:64:1, end:64:11))"]
#[derive(Clone, Debug)]
struct PrepList24Var1 {
    PrepList23: TablePrepList23,
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
        if token_parser.TokenFieldprep31().disassembly() != 1i128 {
            return None;
        }
        let PrepList23 = if let Some((len, table)) = TablePrepList23::parse(
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
        Some((pattern_len, Self { PrepList23 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:65:1, end:65:11))"]
#[derive(Clone, Debug)]
struct PrepList24Var2 {
    PrepList23: TablePrepList23,
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
        if token_parser.TokenFieldprep31().disassembly() != 0i128 {
            return None;
        }
        let PrepList23 = if let Some((len, table)) = TablePrepList23::parse(
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
        Some((pattern_len, Self { PrepList23 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList24 {
    Var0(PrepList24Var0),
    Var1(PrepList24Var1),
    Var2(PrepList24Var2),
}
impl TablePrepList24 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:67:1, end:67:11))"]
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
        if token_parser.TokenFieldprep30().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2427().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldprep31().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:68:1, end:68:11))"]
#[derive(Clone, Debug)]
struct PrepList25Var1 {
    PrepList24: TablePrepList24,
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
        if token_parser.TokenFieldprep30().disassembly() != 1i128 {
            return None;
        }
        let PrepList24 = if let Some((len, table)) = TablePrepList24::parse(
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
        Some((pattern_len, Self { PrepList24 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:69:1, end:69:11))"]
#[derive(Clone, Debug)]
struct PrepList25Var2 {
    PrepList24: TablePrepList24,
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
        if token_parser.TokenFieldprep30().disassembly() != 0i128 {
            return None;
        }
        let PrepList24 = if let Some((len, table)) = TablePrepList24::parse(
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
        Some((pattern_len, Self { PrepList24 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList25 {
    Var0(PrepList25Var0),
    Var1(PrepList25Var1),
    Var2(PrepList25Var2),
}
impl TablePrepList25 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:71:1, end:71:11))"]
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
        if token_parser.TokenFieldprep29().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2427().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldprep3031().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:72:1, end:72:11))"]
#[derive(Clone, Debug)]
struct PrepList26Var1 {
    PrepList25: TablePrepList25,
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
        if token_parser.TokenFieldprep29().disassembly() != 1i128 {
            return None;
        }
        let PrepList25 = if let Some((len, table)) = TablePrepList25::parse(
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
        Some((pattern_len, Self { PrepList25 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:73:1, end:73:11))"]
#[derive(Clone, Debug)]
struct PrepList26Var2 {
    PrepList25: TablePrepList25,
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
        if token_parser.TokenFieldprep29().disassembly() != 0i128 {
            return None;
        }
        let PrepList25 = if let Some((len, table)) = TablePrepList25::parse(
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
        Some((pattern_len, Self { PrepList25 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList26 {
    Var0(PrepList26Var0),
    Var1(PrepList26Var1),
    Var2(PrepList26Var2),
}
impl TablePrepList26 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:75:1, end:75:11))"]
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
        if token_parser.TokenFieldprep28().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2427().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldprep2931().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:76:1, end:76:11))"]
#[derive(Clone, Debug)]
struct PrepList27Var1 {
    PrepList26: TablePrepList26,
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
        if token_parser.TokenFieldprep28().disassembly() != 1i128 {
            return None;
        }
        let PrepList26 = if let Some((len, table)) = TablePrepList26::parse(
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
        Some((pattern_len, Self { PrepList26 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:77:1, end:77:11))"]
#[derive(Clone, Debug)]
struct PrepList27Var2 {
    PrepList26: TablePrepList26,
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
        if token_parser.TokenFieldprep28().disassembly() != 0i128 {
            return None;
        }
        let PrepList26 = if let Some((len, table)) = TablePrepList26::parse(
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
        Some((pattern_len, Self { PrepList26 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList27 {
    Var0(PrepList27Var0),
    Var1(PrepList27Var1),
    Var2(PrepList27Var2),
}
impl TablePrepList27 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:79:1, end:79:11))"]
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
        if token_parser.TokenFieldprep23().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2431().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:80:1, end:80:11))"]
#[derive(Clone, Debug)]
struct PrepList28Var1 {
    PrepList27: TablePrepList27,
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
        if token_parser.TokenFieldprep23().disassembly() != 1i128 {
            return None;
        }
        let PrepList27 = if let Some((len, table)) = TablePrepList27::parse(
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
        Some((pattern_len, Self { PrepList27 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:81:1, end:81:11))"]
#[derive(Clone, Debug)]
struct PrepList28Var2 {
    PrepList27: TablePrepList27,
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
        if token_parser.TokenFieldprep23().disassembly() != 0i128 {
            return None;
        }
        let PrepList27 = if let Some((len, table)) = TablePrepList27::parse(
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
        Some((pattern_len, Self { PrepList27 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList28 {
    Var0(PrepList28Var0),
    Var1(PrepList28Var1),
    Var2(PrepList28Var2),
}
impl TablePrepList28 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:83:1, end:83:11))"]
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
        if token_parser.TokenFieldprep22().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2431().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldprep23().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:84:1, end:84:11))"]
#[derive(Clone, Debug)]
struct PrepList29Var1 {
    PrepList28: TablePrepList28,
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
        if token_parser.TokenFieldprep22().disassembly() != 1i128 {
            return None;
        }
        let PrepList28 = if let Some((len, table)) = TablePrepList28::parse(
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
        Some((pattern_len, Self { PrepList28 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:85:1, end:85:11))"]
#[derive(Clone, Debug)]
struct PrepList29Var2 {
    PrepList28: TablePrepList28,
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
        if token_parser.TokenFieldprep22().disassembly() != 0i128 {
            return None;
        }
        let PrepList28 = if let Some((len, table)) = TablePrepList28::parse(
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
        Some((pattern_len, Self { PrepList28 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList29 {
    Var0(PrepList29Var0),
    Var1(PrepList29Var1),
    Var2(PrepList29Var2),
}
impl TablePrepList29 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:87:1, end:87:11))"]
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
        if token_parser.TokenFieldprep00().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2431().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldprep2223().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:88:1, end:88:11))"]
#[derive(Clone, Debug)]
struct PrepList30Var1 {
    PrepList29: TablePrepList29,
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
        if token_parser.TokenFieldprep00().disassembly() != 1i128 {
            return None;
        }
        let PrepList29 = if let Some((len, table)) = TablePrepList29::parse(
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
        Some((pattern_len, Self { PrepList29 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:89:1, end:89:11))"]
#[derive(Clone, Debug)]
struct PrepList30Var2 {
    PrepList29: TablePrepList29,
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
        if token_parser.TokenFieldprep00().disassembly() != 0i128 {
            return None;
        }
        let PrepList29 = if let Some((len, table)) = TablePrepList29::parse(
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
        Some((pattern_len, Self { PrepList29 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList30 {
    Var0(PrepList30Var0),
    Var1(PrepList30Var1),
    Var2(PrepList30Var2),
}
impl TablePrepList30 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:91:1, end:91:9))"]
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("{"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::lp),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("}"),
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
        if token_parser.TokenFieldprep21().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldprep2431().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldprep2223().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldprep00().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:92:1, end:92:9))"]
#[derive(Clone, Debug)]
struct PrepListVar1 {
    PrepList30: TablePrepList30,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("{"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.PrepList30.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::lp),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("}"),
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
        if token_parser.TokenFieldprep21().disassembly() != 1i128 {
            return None;
        }
        let PrepList30 = if let Some((len, table)) = TablePrepList30::parse(
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
        Some((pattern_len, Self { PrepList30 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:93:1, end:93:9))"]
#[derive(Clone, Debug)]
struct PrepListVar2 {
    PrepList30: TablePrepList30,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("{"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.PrepList30.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(" "), DisplayElement::Literal("}")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldprep21().disassembly() != 0i128 {
            return None;
        }
        let PrepList30 = if let Some((len, table)) = TablePrepList30::parse(
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
        Some((pattern_len, Self { PrepList30 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList {
    Var0(PrepListVar0),
    Var1(PrepListVar1),
    Var2(PrepListVar2),
}
impl TablePrepList {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:97:1, end:97:11))"]
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
        if token_parser.TokenFieldprep21().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:98:1, end:98:11))"]
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
        if token_parser.TokenFieldprep21().disassembly() != 0i128 {
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
enum TableDispList31 {
    Var0(DispList31Var0),
    Var1(DispList31Var1),
}
impl TableDispList31 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:100:1, end:100:11))"]
#[derive(Clone, Debug)]
struct DispList30Var0 {
    DispList31: TableDispList31,
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
        if token_parser.TokenFieldprep00().disassembly() != 1i128 {
            return None;
        }
        let DispList31 = if let Some((len, table)) = TableDispList31::parse(
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
        Some((pattern_len, Self { DispList31 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:101:1, end:101:11))"]
#[derive(Clone, Debug)]
struct DispList30Var1 {
    DispList31: TableDispList31,
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
        if token_parser.TokenFieldprep00().disassembly() != 0i128 {
            return None;
        }
        let DispList31 = if let Some((len, table)) = TableDispList31::parse(
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
        Some((pattern_len, Self { DispList31 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList30 {
    Var0(DispList30Var0),
    Var1(DispList30Var1),
}
impl TableDispList30 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:103:1, end:103:11))"]
#[derive(Clone, Debug)]
struct DispList29Var0 {
    DispList30: TableDispList30,
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
        if token_parser.TokenFieldprep22().disassembly() != 1i128 {
            return None;
        }
        let DispList30 = if let Some((len, table)) = TableDispList30::parse(
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
        Some((pattern_len, Self { DispList30 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:104:1, end:104:11))"]
#[derive(Clone, Debug)]
struct DispList29Var1 {
    DispList30: TableDispList30,
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
        if token_parser.TokenFieldprep22().disassembly() != 0i128 {
            return None;
        }
        let DispList30 = if let Some((len, table)) = TableDispList30::parse(
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
        Some((pattern_len, Self { DispList30 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList29 {
    Var0(DispList29Var0),
    Var1(DispList29Var1),
}
impl TableDispList29 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:106:1, end:106:11))"]
#[derive(Clone, Debug)]
struct DispList28Var0 {
    DispList29: TableDispList29,
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
        if token_parser.TokenFieldprep23().disassembly() != 1i128 {
            return None;
        }
        let DispList29 = if let Some((len, table)) = TableDispList29::parse(
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
        Some((pattern_len, Self { DispList29 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:107:1, end:107:11))"]
#[derive(Clone, Debug)]
struct DispList28Var1 {
    DispList29: TableDispList29,
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
        if token_parser.TokenFieldprep23().disassembly() != 0i128 {
            return None;
        }
        let DispList29 = if let Some((len, table)) = TableDispList29::parse(
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
        Some((pattern_len, Self { DispList29 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList28 {
    Var0(DispList28Var0),
    Var1(DispList28Var1),
}
impl TableDispList28 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:109:1, end:109:11))"]
#[derive(Clone, Debug)]
struct DispList27Var0 {
    DispList28: TableDispList28,
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
        if token_parser.TokenFieldprep28().disassembly() != 1i128 {
            return None;
        }
        let DispList28 = if let Some((len, table)) = TableDispList28::parse(
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
        Some((pattern_len, Self { DispList28 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:110:1, end:110:11))"]
#[derive(Clone, Debug)]
struct DispList27Var1 {
    DispList28: TableDispList28,
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
        if token_parser.TokenFieldprep28().disassembly() != 0i128 {
            return None;
        }
        let DispList28 = if let Some((len, table)) = TableDispList28::parse(
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
        Some((pattern_len, Self { DispList28 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList27 {
    Var0(DispList27Var0),
    Var1(DispList27Var1),
}
impl TableDispList27 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:112:1, end:112:11))"]
#[derive(Clone, Debug)]
struct DispList26Var0 {
    DispList27: TableDispList27,
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
        if token_parser.TokenFieldprep29().disassembly() != 1i128 {
            return None;
        }
        let DispList27 = if let Some((len, table)) = TableDispList27::parse(
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
        Some((pattern_len, Self { DispList27 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:113:1, end:113:11))"]
#[derive(Clone, Debug)]
struct DispList26Var1 {
    DispList27: TableDispList27,
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
        if token_parser.TokenFieldprep29().disassembly() != 0i128 {
            return None;
        }
        let DispList27 = if let Some((len, table)) = TableDispList27::parse(
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
        Some((pattern_len, Self { DispList27 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList26 {
    Var0(DispList26Var0),
    Var1(DispList26Var1),
}
impl TableDispList26 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:115:1, end:115:11))"]
#[derive(Clone, Debug)]
struct DispList25Var0 {
    DispList26: TableDispList26,
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
        if token_parser.TokenFieldprep30().disassembly() != 1i128 {
            return None;
        }
        let DispList26 = if let Some((len, table)) = TableDispList26::parse(
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
        Some((pattern_len, Self { DispList26 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:116:1, end:116:11))"]
#[derive(Clone, Debug)]
struct DispList25Var1 {
    DispList26: TableDispList26,
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
        if token_parser.TokenFieldprep30().disassembly() != 0i128 {
            return None;
        }
        let DispList26 = if let Some((len, table)) = TableDispList26::parse(
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
        Some((pattern_len, Self { DispList26 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList25 {
    Var0(DispList25Var0),
    Var1(DispList25Var1),
}
impl TableDispList25 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:118:1, end:118:11))"]
#[derive(Clone, Debug)]
struct DispList24Var0 {
    DispList25: TableDispList25,
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
        if token_parser.TokenFieldprep31().disassembly() != 1i128 {
            return None;
        }
        let DispList25 = if let Some((len, table)) = TableDispList25::parse(
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
        Some((pattern_len, Self { DispList25 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:119:1, end:119:11))"]
#[derive(Clone, Debug)]
struct DispList24Var1 {
    DispList25: TableDispList25,
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
        if token_parser.TokenFieldprep31().disassembly() != 0i128 {
            return None;
        }
        let DispList25 = if let Some((len, table)) = TableDispList25::parse(
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
        Some((pattern_len, Self { DispList25 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList24 {
    Var0(DispList24Var0),
    Var1(DispList24Var1),
}
impl TableDispList24 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:121:1, end:121:11))"]
#[derive(Clone, Debug)]
struct DispList23Var0 {
    DispList24: TableDispList24,
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
        if token_parser.TokenFieldprep24().disassembly() != 1i128 {
            return None;
        }
        let DispList24 = if let Some((len, table)) = TableDispList24::parse(
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
        Some((pattern_len, Self { DispList24 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:122:1, end:122:11))"]
#[derive(Clone, Debug)]
struct DispList23Var1 {
    DispList24: TableDispList24,
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
        if token_parser.TokenFieldprep24().disassembly() != 0i128 {
            return None;
        }
        let DispList24 = if let Some((len, table)) = TableDispList24::parse(
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
        Some((pattern_len, Self { DispList24 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList23 {
    Var0(DispList23Var0),
    Var1(DispList23Var1),
}
impl TableDispList23 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:124:1, end:124:11))"]
#[derive(Clone, Debug)]
struct DispList22Var0 {
    DispList23: TableDispList23,
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
        if token_parser.TokenFieldprep25().disassembly() != 1i128 {
            return None;
        }
        let DispList23 = if let Some((len, table)) = TableDispList23::parse(
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
        Some((pattern_len, Self { DispList23 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:125:1, end:125:11))"]
#[derive(Clone, Debug)]
struct DispList22Var1 {
    DispList23: TableDispList23,
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
        if token_parser.TokenFieldprep25().disassembly() != 0i128 {
            return None;
        }
        let DispList23 = if let Some((len, table)) = TableDispList23::parse(
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
        Some((pattern_len, Self { DispList23 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList22 {
    Var0(DispList22Var0),
    Var1(DispList22Var1),
}
impl TableDispList22 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:127:1, end:127:11))"]
#[derive(Clone, Debug)]
struct DispList21Var0 {
    DispList22: TableDispList22,
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
        if token_parser.TokenFieldprep26().disassembly() != 1i128 {
            return None;
        }
        let DispList22 = if let Some((len, table)) = TableDispList22::parse(
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
        Some((pattern_len, Self { DispList22 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:128:1, end:128:11))"]
#[derive(Clone, Debug)]
struct DispList21Var1 {
    DispList22: TableDispList22,
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
        if token_parser.TokenFieldprep26().disassembly() != 0i128 {
            return None;
        }
        let DispList22 = if let Some((len, table)) = TableDispList22::parse(
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
        Some((pattern_len, Self { DispList22 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList21 {
    Var0(DispList21Var0),
    Var1(DispList21Var1),
}
impl TableDispList21 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:130:1, end:130:9))"]
#[derive(Clone, Debug)]
struct DispListVar0 {
    DispList21: TableDispList21,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("{"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r20),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList21.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(" "), DisplayElement::Literal("}")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldprep27().disassembly() != 1i128 {
            return None;
        }
        let DispList21 = if let Some((len, table)) = TableDispList21::parse(
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
        Some((pattern_len, Self { DispList21 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:131:1, end:131:9))"]
#[derive(Clone, Debug)]
struct DispListVar1 {
    DispList21: TableDispList21,
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
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("{"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.DispList21.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(" "), DisplayElement::Literal("}")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldprep27().disassembly() != 0i128 {
            return None;
        }
        let DispList21 = if let Some((len, table)) = TableDispList21::parse(
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
        Some((pattern_len, Self { DispList21 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList {
    Var0(DispListVar0),
    Var1(DispListVar1),
}
impl TableDispList {
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
