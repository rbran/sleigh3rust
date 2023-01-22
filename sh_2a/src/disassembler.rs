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
        0 => Register::fr0,
        1 => Register::fr1,
        2 => Register::fr2,
        3 => Register::fr3,
        4 => Register::fr4,
        5 => Register::fr5,
        6 => Register::fr6,
        7 => Register::fr7,
        8 => Register::fr8,
        9 => Register::fr9,
        10 => Register::fr10,
        11 => Register::fr11,
        12 => Register::fr12,
        13 => Register::fr13,
        14 => Register::fr14,
        15 => Register::fr15,
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
fn meaning_5_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_5_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_5_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::dr0,
        1 => Register::dr2,
        2 => Register::dr4,
        3 => Register::dr6,
        4 => Register::dr8,
        5 => Register::dr10,
        6 => Register::dr12,
        7 => Register::dr14,
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
#[derive(Clone, Copy, Debug)]
struct TokenField_l_disp_00_11(u16);
impl TokenField_l_disp_00_11 {
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
struct TokenField_l_opcode_12_15(u8);
impl TokenField_l_opcode_12_15 {
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
struct TokenField_l_opcode_16_19(u8);
impl TokenField_l_opcode_16_19 {
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
struct TokenField_l_opcode_23_23(u8);
impl TokenField_l_opcode_23_23 {
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
struct TokenField_l_opcode_24_31(u8);
impl TokenField_l_opcode_24_31 {
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
struct TokenField_l_rm_20_23(u8);
impl TokenField_l_rm_20_23 {
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
struct TokenField_l_rn_24_27(u8);
impl TokenField_l_rn_24_27 {
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
struct TokenField_l_opcode_28_31(u8);
impl TokenField_l_opcode_28_31 {
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
struct TokenField_l_imm20_00_15(u16);
impl TokenField_l_imm20_00_15 {
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
struct TokenField_l_simm20_20_23(i8);
impl TokenField_l_simm20_20_23 {
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
struct TokenField_l_imm3_20_22(u8);
impl TokenField_l_imm3_20_22 {
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
struct TokenField_fop_00_07(u8);
impl TokenField_fop_00_07 {
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
struct TokenField_fop_00_03(u8);
impl TokenField_fop_00_03 {
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
struct TokenField_fop_04_07(u8);
impl TokenField_fop_04_07 {
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
struct TokenField_fop_12_15(u8);
impl TokenField_fop_12_15 {
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
struct TokenField_fop_08_08(u8);
impl TokenField_fop_08_08 {
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
struct TokenField_fop_04_04(u8);
impl TokenField_fop_04_04 {
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
struct TokenField_fop_00_15(u16);
impl TokenField_fop_00_15 {
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
struct TokenField_ffrn_08_11(u8);
impl TokenField_ffrn_08_11 {
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
struct TokenField_ffrm_08_11(u8);
impl TokenField_ffrm_08_11 {
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
struct TokenField_ffrn_04_07(u8);
impl TokenField_ffrn_04_07 {
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
struct TokenField_ffrm_04_07(u8);
impl TokenField_ffrm_04_07 {
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
struct TokenField_f_rm_04_07(u8);
impl TokenField_f_rm_04_07 {
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
struct TokenField_f_rn_08_11(u8);
impl TokenField_f_rn_08_11 {
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
struct TokenField_fdrn_09_11(u8);
impl TokenField_fdrn_09_11 {
    fn execution(&self) -> Register {
        meaning_5_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_5_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_fdrm_09_11(u8);
impl TokenField_fdrm_09_11 {
    fn execution(&self) -> Register {
        meaning_5_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_5_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_fdrn_05_07(u8);
impl TokenField_fdrn_05_07 {
    fn execution(&self) -> Register {
        meaning_5_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_5_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_fdrm_05_07(u8);
impl TokenField_fdrm_05_07 {
    fn execution(&self) -> Register {
        meaning_5_value(self.0)
    }
    fn disassembly(&self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_5_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_lfdisp_00_11(u16);
impl TokenField_lfdisp_00_11 {
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
struct TokenField_lfop_28_31(u8);
impl TokenField_lfop_28_31 {
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
struct TokenField_lfop_12_19(u8);
impl TokenField_lfop_12_19 {
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
struct TokenField_lffrm_24_27(u8);
impl TokenField_lffrm_24_27 {
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
struct TokenField_lffrn_24_27(u8);
impl TokenField_lffrn_24_27 {
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
struct TokenField_lffrm_20_23(u8);
impl TokenField_lffrm_20_23 {
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
struct TokenField_lf_rm_20_23(u8);
impl TokenField_lf_rm_20_23 {
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
struct TokenField_lf_rn_24_27(u8);
impl TokenField_lf_rn_24_27 {
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
struct TokenField_lffrn_20_23(u8);
impl TokenField_lffrn_20_23 {
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
    fn TokenFielddisp_00_03(&self) -> TokenField_disp_00_03 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_disp_00_03(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsdisp_00_03(&self) -> TokenField_sdisp_00_03 {
        let inner_value = self.read_i8::<true>(1, 0, 4).unwrap();
        TokenField_sdisp_00_03(i8::try_from(inner_value).unwrap())
    }
    fn TokenFielddisp_00_07(&self) -> TokenField_disp_00_07 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_disp_00_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsdisp_00_07(&self) -> TokenField_sdisp_00_07 {
        let inner_value = self.read_i8::<true>(1, 0, 8).unwrap();
        TokenField_sdisp_00_07(i8::try_from(inner_value).unwrap())
    }
    fn TokenFielddisp_00_11(&self) -> TokenField_disp_00_11 {
        let inner_value = self.read_u16::<true>(0, 0, 12).unwrap();
        TokenField_disp_00_11(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldsdisp_00_11(&self) -> TokenField_sdisp_00_11 {
        let inner_value = self.read_i16::<true>(0, 0, 12).unwrap();
        TokenField_sdisp_00_11(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm3_00_02(&self) -> TokenField_imm3_00_02 {
        let inner_value = self.read_u8::<true>(1, 0, 3).unwrap();
        TokenField_imm3_00_02(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm_00_07(&self) -> TokenField_imm_00_07 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_imm_00_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm_00_07(&self) -> TokenField_simm_00_07 {
        let inner_value = self.read_i8::<true>(1, 0, 8).unwrap();
        TokenField_simm_00_07(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_00_03(&self) -> TokenField_opcode_00_03 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_opcode_00_03(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_00_07(&self) -> TokenField_opcode_00_07 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_opcode_00_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_00_15(&self) -> TokenField_opcode_00_15 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_opcode_00_15(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_03_03(&self) -> TokenField_opcode_03_03 {
        let inner_value = self.read_u8::<true>(1, 3, 1).unwrap();
        TokenField_opcode_03_03(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_04_07(&self) -> TokenField_opcode_04_07 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_opcode_04_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_08_11(&self) -> TokenField_opcode_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_opcode_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_08_15(&self) -> TokenField_opcode_08_15 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_opcode_08_15(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode_12_15(&self) -> TokenField_opcode_12_15 {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_opcode_12_15(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrm_04_07(&self) -> TokenField_rm_04_07 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_rm_04_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrm_08_11(&self) -> TokenField_rm_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_rm_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrn_04_07(&self) -> TokenField_rn_04_07 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_rn_04_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrn_08_11(&self) -> TokenField_rn_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_rn_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrm_imm_08_11(&self) -> TokenField_rm_imm_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_rm_imm_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrn_imm_08_11(&self) -> TokenField_rn_imm_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_rn_imm_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_disp_00_11(&self) -> TokenField_l_disp_00_11 {
        let inner_value = self.read_u16::<true>(2, 0, 12).unwrap();
        TokenField_l_disp_00_11(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_opcode_12_15(&self) -> TokenField_l_opcode_12_15 {
        let inner_value = self.read_u8::<true>(2, 4, 4).unwrap();
        TokenField_l_opcode_12_15(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_opcode_16_19(&self) -> TokenField_l_opcode_16_19 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_l_opcode_16_19(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_opcode_23_23(&self) -> TokenField_l_opcode_23_23 {
        let inner_value = self.read_u8::<true>(1, 7, 1).unwrap();
        TokenField_l_opcode_23_23(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_opcode_24_31(&self) -> TokenField_l_opcode_24_31 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_l_opcode_24_31(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_rm_20_23(&self) -> TokenField_l_rm_20_23 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_l_rm_20_23(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_rn_24_27(&self) -> TokenField_l_rn_24_27 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_l_rn_24_27(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_opcode_28_31(&self) -> TokenField_l_opcode_28_31 {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_l_opcode_28_31(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_imm20_00_15(&self) -> TokenField_l_imm20_00_15 {
        let inner_value = self.read_u16::<true>(2, 0, 16).unwrap();
        TokenField_l_imm20_00_15(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_simm20_20_23(&self) -> TokenField_l_simm20_20_23 {
        let inner_value = self.read_i8::<true>(1, 4, 4).unwrap();
        TokenField_l_simm20_20_23(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldl_imm3_20_22(&self) -> TokenField_l_imm3_20_22 {
        let inner_value = self.read_u8::<true>(1, 4, 3).unwrap();
        TokenField_l_imm3_20_22(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfop_00_07(&self) -> TokenField_fop_00_07 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_fop_00_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfop_00_03(&self) -> TokenField_fop_00_03 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_fop_00_03(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfop_04_07(&self) -> TokenField_fop_04_07 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_fop_04_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfop_12_15(&self) -> TokenField_fop_12_15 {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_fop_12_15(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfop_08_08(&self) -> TokenField_fop_08_08 {
        let inner_value = self.read_u8::<true>(0, 0, 1).unwrap();
        TokenField_fop_08_08(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfop_04_04(&self) -> TokenField_fop_04_04 {
        let inner_value = self.read_u8::<true>(1, 4, 1).unwrap();
        TokenField_fop_04_04(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfop_00_15(&self) -> TokenField_fop_00_15 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_fop_00_15(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldffrn_08_11(&self) -> TokenField_ffrn_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_ffrn_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldffrm_08_11(&self) -> TokenField_ffrm_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_ffrm_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldffrn_04_07(&self) -> TokenField_ffrn_04_07 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_ffrn_04_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldffrm_04_07(&self) -> TokenField_ffrm_04_07 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_ffrm_04_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldf_rm_04_07(&self) -> TokenField_f_rm_04_07 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_f_rm_04_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldf_rn_08_11(&self) -> TokenField_f_rn_08_11 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_f_rn_08_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfdrn_09_11(&self) -> TokenField_fdrn_09_11 {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_fdrn_09_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfdrm_09_11(&self) -> TokenField_fdrm_09_11 {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_fdrm_09_11(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfdrn_05_07(&self) -> TokenField_fdrn_05_07 {
        let inner_value = self.read_u8::<true>(1, 5, 3).unwrap();
        TokenField_fdrn_05_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldfdrm_05_07(&self) -> TokenField_fdrm_05_07 {
        let inner_value = self.read_u8::<true>(1, 5, 3).unwrap();
        TokenField_fdrm_05_07(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlfdisp_00_11(&self) -> TokenField_lfdisp_00_11 {
        let inner_value = self.read_u16::<true>(2, 0, 12).unwrap();
        TokenField_lfdisp_00_11(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldlfop_28_31(&self) -> TokenField_lfop_28_31 {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_lfop_28_31(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlfop_12_19(&self) -> TokenField_lfop_12_19 {
        let inner_value = self.read_u16::<true>(1, 4, 8).unwrap();
        TokenField_lfop_12_19(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlffrm_24_27(&self) -> TokenField_lffrm_24_27 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_lffrm_24_27(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlffrn_24_27(&self) -> TokenField_lffrn_24_27 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_lffrn_24_27(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlffrm_20_23(&self) -> TokenField_lffrm_20_23 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_lffrm_20_23(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlf_rm_20_23(&self) -> TokenField_lf_rm_20_23 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_lf_rm_20_23(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlf_rn_24_27(&self) -> TokenField_lf_rn_24_27 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_lf_rn_24_27(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlffrn_20_23(&self) -> TokenField_lffrn_20_23 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_lffrn_20_23(u8::try_from(inner_value).unwrap())
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
    tbr,
    fr0,
    fr1,
    fr2,
    fr3,
    fr4,
    fr5,
    fr6,
    fr7,
    fr8,
    fr9,
    fr10,
    fr11,
    fr12,
    fr13,
    fr14,
    fr15,
    dr0,
    dr2,
    dr4,
    dr6,
    dr8,
    dr10,
    dr12,
    dr14,
    fpscr,
    fpul,
    resbank_base,
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
            Self::tbr => write!(f, "tbr"),
            Self::fr0 => write!(f, "fr0"),
            Self::fr1 => write!(f, "fr1"),
            Self::fr2 => write!(f, "fr2"),
            Self::fr3 => write!(f, "fr3"),
            Self::fr4 => write!(f, "fr4"),
            Self::fr5 => write!(f, "fr5"),
            Self::fr6 => write!(f, "fr6"),
            Self::fr7 => write!(f, "fr7"),
            Self::fr8 => write!(f, "fr8"),
            Self::fr9 => write!(f, "fr9"),
            Self::fr10 => write!(f, "fr10"),
            Self::fr11 => write!(f, "fr11"),
            Self::fr12 => write!(f, "fr12"),
            Self::fr13 => write!(f, "fr13"),
            Self::fr14 => write!(f, "fr14"),
            Self::fr15 => write!(f, "fr15"),
            Self::dr0 => write!(f, "dr0"),
            Self::dr2 => write!(f, "dr2"),
            Self::dr4 => write!(f, "dr4"),
            Self::dr6 => write!(f, "dr6"),
            Self::dr8 => write!(f, "dr8"),
            Self::dr10 => write!(f, "dr10"),
            Self::dr12 => write!(f, "dr12"),
            Self::dr14 => write!(f, "dr14"),
            Self::fpscr => write!(f, "fpscr"),
            Self::fpul => write!(f, "fpul"),
            Self::resbank_base => write!(f, "resbank_base"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:821:1"]
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
            [DisplayElement::Literal("nott")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 104i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1011:1"]
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1713:1"]
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("rts"),
            DisplayElement::Literal("/n"),
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
        if token_parser.TokenFieldopcode_00_15().disassembly() != 107i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1767:1"]
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
            [DisplayElement::Literal("resbank")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode_00_15().disassembly() != 91i64 {
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
struct instructionVar8 {}
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
struct instructionVar9 {}
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
struct instructionVar10 {}
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2177:1"]
#[derive(Clone, Debug)]
struct instructionVar11 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fschg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldfop_00_15().disassembly() != 62461i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2031:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {
    fdrm_09_11: TokenField_fdrm_09_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fcnvds"),
            DisplayElement::Literal(" "),
            self.fdrm_09_11.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::fpul),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_08_08().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 189i64 {
            return None;
        }
        let fdrm_09_11 = token_parser.TokenFieldfdrm_09_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fdrm_09_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2037:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {
    fdrn_09_11: TokenField_fdrn_09_11,
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
            DisplayElement::Literal("fcnvsd"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fpul),
            DisplayElement::Literal(", "),
            self.fdrn_09_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_08_08().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 173i64 {
            return None;
        }
        let fdrn_09_11 = token_parser.TokenFieldfdrn_09_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fdrn_09_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:245:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {
    rm_04_07: TokenField_rm_04_07,
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
struct instructionVar15 {
    rm_04_07: TokenField_rm_04_07,
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
struct instructionVar16 {
    rm_04_07: TokenField_rm_04_07,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:396:1"]
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", @"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 139i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:404:1"]
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", @"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 155i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:412:1"]
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", @"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 171i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:420:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" @-"),
            self.rm_08_11.display(),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 203i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:428:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" @-"),
            self.rm_08_11.display(),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 219i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:436:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" @-"),
            self.rm_08_11.display(),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 235i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:606:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {
    MovMLReg1: TableMovMLReg1,
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
            DisplayElement::Literal("movml.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.MovMLReg1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(", @-"),
            DisplayElement::Register(Register::r15),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 241i64 {
            return None;
        }
        let MovMLReg1 = if let Some((len, table)) = TableMovMLReg1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:691:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {
    MovMLReg2: TableMovMLReg2,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("movml.l"),
            DisplayElement::Literal(" @"),
            DisplayElement::Register(Register::r15),
            DisplayElement::Literal("+, "),
        ];
        display.extend_from_slice(&extend);
        self.MovMLReg2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 245i64 {
            return None;
        }
        let MovMLReg2 = if let Some((len, table)) = TableMovMLReg2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn_imm_08_11 = token_parser.TokenFieldrn_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:749:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {
    MovMUReg1: TableMovMUReg1,
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
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("movmu.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.MovMUReg1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(", @-"),
            DisplayElement::Register(Register::r15),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 240i64 {
            return None;
        }
        let MovMUReg1 = if let Some((len, table)) = TableMovMUReg1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:807:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {
    MovMUReg2: TableMovMUReg2,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("movmu.l"),
            DisplayElement::Literal(" @"),
            DisplayElement::Register(Register::r15),
            DisplayElement::Literal("+, "),
        ];
        display.extend_from_slice(&extend);
        self.MovMUReg2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 244i64 {
            return None;
        }
        let MovMUReg2 = if let Some((len, table)) = TableMovMUReg2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rn_imm_08_11 = token_parser.TokenFieldrn_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:815:1"]
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
            DisplayElement::Literal("movrt"),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 57i64 {
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
struct instructionVar29 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar30 {
    rn_08_11: TokenField_rn_08_11,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:967:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("clips.b"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 145i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:976:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("clips.w"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 149i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:985:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("clipu.b"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 129i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:993:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("clipu.w"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 133i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1048:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {
    rn_08_11: TokenField_rn_08_11,
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
            DisplayElement::Literal("divs"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 148i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1053:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {
    rn_08_11: TokenField_rn_08_11,
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
            DisplayElement::Literal("divu"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 132i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1081:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("dt"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 16i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1300:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {
    rn_08_11: TokenField_rn_08_11,
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
            DisplayElement::Literal("mulr"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 128i64 {
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
struct instructionVar39 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar40 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar41 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar42 {
    rn_08_11: TokenField_rn_08_11,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1634:1"]
#[derive(Clone, Debug)]
struct instructionVar54 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("braf"),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 35i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1653:1"]
#[derive(Clone, Debug)]
struct instructionVar55 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("bsrf"),
            DisplayElement::Literal("   "),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 3i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1665:1"]
#[derive(Clone, Debug)]
struct instructionVar56 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar57 {
    rm_08_11: TokenField_rm_08_11,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1696:1"]
#[derive(Clone, Debug)]
struct instructionVar58 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("jsr"),
            DisplayElement::Literal("/n"),
            DisplayElement::Literal(" @"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 75i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1719:1"]
#[derive(Clone, Debug)]
struct instructionVar59 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("rtv"),
            DisplayElement::Literal("/n"),
            DisplayElement::Literal(" "),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 123i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1743:1"]
#[derive(Clone, Debug)]
struct instructionVar60 {
    rm_08_11: TokenField_rm_08_11,
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
            DisplayElement::Literal("ldbank"),
            DisplayElement::Literal(" @"),
            self.rm_08_11.display(),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_00_07().disassembly() != 229i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1755:1"]
#[derive(Clone, Debug)]
struct instructionVar61 {
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
            DisplayElement::Literal("stbank"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", @"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 225i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1775:1"]
#[derive(Clone, Debug)]
struct instructionVar62 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar63 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar64 {
    rm_08_11: TokenField_rm_08_11,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1793:1"]
#[derive(Clone, Debug)]
struct instructionVar65 {
    rm_08_11: TokenField_rm_08_11,
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
            DisplayElement::Literal("ldc"),
            DisplayElement::Literal("    "),
            self.rm_08_11.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::tbr),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 74i64 {
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
struct instructionVar66 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar67 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar68 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar69 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar70 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar71 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar72 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar73 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar74 {
    rm_08_11: TokenField_rm_08_11,
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
struct instructionVar75 {
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
struct instructionVar76 {
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
struct instructionVar77 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1911:1"]
#[derive(Clone, Debug)]
struct instructionVar78 {
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("stc"),
            DisplayElement::Literal("    "),
            DisplayElement::Register(Register::tbr),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 74i64 {
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
struct instructionVar79 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar80 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar81 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar82 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar83 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar84 {
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar85 {
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
struct instructionVar86 {
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
struct instructionVar87 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1999:1"]
#[derive(Clone, Debug)]
struct instructionVar88 {
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fabs"),
            DisplayElement::Literal(" "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 93i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2051:1"]
#[derive(Clone, Debug)]
struct instructionVar89 {
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fldi0"),
            DisplayElement::Literal(" "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 141i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2057:1"]
#[derive(Clone, Debug)]
struct instructionVar90 {
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fldi1"),
            DisplayElement::Literal(" "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 157i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2064:1"]
#[derive(Clone, Debug)]
struct instructionVar91 {
    ffrm_08_11: TokenField_ffrm_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("flds"),
            DisplayElement::Literal(" "),
            self.ffrm_08_11.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::fpul),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 29i64 {
            return None;
        }
        let ffrm_08_11 = token_parser.TokenFieldffrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2070:1"]
#[derive(Clone, Debug)]
struct instructionVar92 {
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("float"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fpul),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 45i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2170:1"]
#[derive(Clone, Debug)]
struct instructionVar93 {
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fneg"),
            DisplayElement::Literal(" "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 77i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2183:1"]
#[derive(Clone, Debug)]
struct instructionVar94 {
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fsqrt"),
            DisplayElement::Literal(" "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 109i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2190:1"]
#[derive(Clone, Debug)]
struct instructionVar95 {
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fsts"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fpul),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 13i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2204:1"]
#[derive(Clone, Debug)]
struct instructionVar96 {
    ffrm_08_11: TokenField_ffrm_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ftrc"),
            DisplayElement::Literal(" "),
            self.ffrm_08_11.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::fpul),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_07().disassembly() != 61i64 {
            return None;
        }
        let ffrm_08_11 = token_parser.TokenFieldffrm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrm_08_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2221:1"]
#[derive(Clone, Debug)]
struct instructionVar97 {
    rm_08_11: TokenField_rm_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lds"),
            DisplayElement::Literal(" "),
            self.rm_08_11.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::fpscr),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 106i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2227:1"]
#[derive(Clone, Debug)]
struct instructionVar98 {
    rm_08_11: TokenField_rm_08_11,
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
            DisplayElement::Literal("lds"),
            DisplayElement::Literal(" "),
            self.rm_08_11.display(),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::fpul),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 90i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2233:1"]
#[derive(Clone, Debug)]
struct instructionVar99 {
    rm_08_11: TokenField_rm_08_11,
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
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+, "),
            DisplayElement::Register(Register::fpscr),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 102i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2240:1"]
#[derive(Clone, Debug)]
struct instructionVar100 {
    rm_08_11: TokenField_rm_08_11,
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
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" @"),
            self.rm_08_11.display(),
            DisplayElement::Literal("+, "),
            DisplayElement::Register(Register::fpul),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 86i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2247:1"]
#[derive(Clone, Debug)]
struct instructionVar101 {
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
            DisplayElement::Literal("sts"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fpscr),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 106i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2253:1"]
#[derive(Clone, Debug)]
struct instructionVar102 {
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
            DisplayElement::Literal("sts"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fpul),
            DisplayElement::Literal(", "),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 90i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2259:1"]
#[derive(Clone, Debug)]
struct instructionVar103 {
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
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fpscr),
            DisplayElement::Literal(", @-"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 98i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2266:1"]
#[derive(Clone, Debug)]
struct instructionVar104 {
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fpul),
            DisplayElement::Literal(", @-"),
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
        if token_parser.TokenFieldopcode_00_07().disassembly() != 82i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2316:1"]
#[derive(Clone, Debug)]
struct instructionVar106 {
    imm3_00_02: TokenField_imm3_00_02,
    rn_04_07: TokenField_rn_04_07,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("bclr"),
            DisplayElement::Literal("   "),
            DisplayElement::Literal("#"),
            self.imm3_00_02.display(),
            DisplayElement::Literal(", "),
            self.rn_04_07.display(),
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
        if token_parser.TokenFieldopcode_08_15().disassembly() != 134i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_03_03().disassembly() != 0i64 {
            return None;
        }
        let rn_04_07 = token_parser.TokenFieldrn_04_07();
        let imm3_00_02 = token_parser.TokenFieldimm3_00_02();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_04_07,
                imm3_00_02,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2332:1"]
#[derive(Clone, Debug)]
struct instructionVar107 {
    imm3_00_02: TokenField_imm3_00_02,
    rn_04_07: TokenField_rn_04_07,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("bld"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.imm3_00_02.display(),
            DisplayElement::Literal(", "),
            self.rn_04_07.display(),
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
        if token_parser.TokenFieldopcode_08_15().disassembly() != 135i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_03_03().disassembly() != 1i64 {
            return None;
        }
        let rn_04_07 = token_parser.TokenFieldrn_04_07();
        let imm3_00_02 = token_parser.TokenFieldimm3_00_02();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_04_07,
                imm3_00_02,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2375:1"]
#[derive(Clone, Debug)]
struct instructionVar108 {
    imm3_00_02: TokenField_imm3_00_02,
    rn_04_07: TokenField_rn_04_07,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("bset"),
            DisplayElement::Literal("       "),
            DisplayElement::Literal("#"),
            self.imm3_00_02.display(),
            DisplayElement::Literal(", "),
            self.rn_04_07.display(),
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
        if token_parser.TokenFieldopcode_08_15().disassembly() != 134i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_03_03().disassembly() != 1i64 {
            return None;
        }
        let rn_04_07 = token_parser.TokenFieldrn_04_07();
        let imm3_00_02 = token_parser.TokenFieldimm3_00_02();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_04_07,
                imm3_00_02,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2392:1"]
#[derive(Clone, Debug)]
struct instructionVar109 {
    imm3_00_02: TokenField_imm3_00_02,
    rn_04_07: TokenField_rn_04_07,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("bst"),
            DisplayElement::Literal("        "),
            DisplayElement::Literal("#"),
            self.imm3_00_02.display(),
            DisplayElement::Literal(", "),
            self.rn_04_07.display(),
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
        if token_parser.TokenFieldopcode_08_15().disassembly() != 135i64 {
            return None;
        }
        if token_parser.TokenFieldopcode_03_03().disassembly() != 0i64 {
            return None;
        }
        let rn_04_07 = token_parser.TokenFieldrn_04_07();
        let imm3_00_02 = token_parser.TokenFieldimm3_00_02();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_04_07,
                imm3_00_02,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:199:1"]
#[derive(Clone, Debug)]
struct instructionVar110 {
    disppc4: Tabledisppc4,
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
struct instructionVar113 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar114 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar117 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar118 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar120 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar121 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar123 {
    disp_00_03: TokenField_disp_00_03,
    rm_04_07: TokenField_rm_04_07,
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
struct instructionVar124 {
    rm_04_07: TokenField_rm_04_07,
    disp_00_03: TokenField_disp_00_03,
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
struct instructionVar125 {
    disp_00_03: TokenField_disp_00_03,
    rn_04_07: TokenField_rn_04_07,
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
struct instructionVar126 {
    rn_04_07: TokenField_rn_04_07,
    disp_00_03: TokenField_disp_00_03,
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
struct instructionVar127 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar128 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar129 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar130 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar131 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar132 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar133 {
    disp_00_07: TokenField_disp_00_07,
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
struct instructionVar134 {
    disp_00_07: TokenField_disp_00_07,
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
struct instructionVar135 {
    disp_00_07: TokenField_disp_00_07,
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
struct instructionVar136 {
    disp_00_07: TokenField_disp_00_07,
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
struct instructionVar137 {
    disp_00_07: TokenField_disp_00_07,
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
struct instructionVar138 {
    disp_00_07: TokenField_disp_00_07,
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
struct instructionVar139 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar140 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar141 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar142 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar143 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar144 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar145 {
    simm_00_07: TokenField_simm_00_07,
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
struct instructionVar146 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar147 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar148 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar149 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar150 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar151 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar152 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar153 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1061:1"]
#[derive(Clone, Debug)]
struct instructionVar154 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("dmuls.l"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1071:1"]
#[derive(Clone, Debug)]
struct instructionVar155 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("dmulu.l"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1088:1"]
#[derive(Clone, Debug)]
struct instructionVar156 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar157 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar158 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar159 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1111:1"]
#[derive(Clone, Debug)]
struct instructionVar160 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
            DisplayElement::Literal("mac.l"),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1220:1"]
#[derive(Clone, Debug)]
struct instructionVar161 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1293:1"]
#[derive(Clone, Debug)]
struct instructionVar162 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mul.l"),
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
        if token_parser.TokenFieldopcode_12_15().disassembly() != 0i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1306:1"]
#[derive(Clone, Debug)]
struct instructionVar163 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar164 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar165 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar166 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar167 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar168 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar169 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar170 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar171 {
    imm_00_07: TokenField_imm_00_07,
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
struct instructionVar172 {
    imm_00_07: TokenField_imm_00_07,
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
struct instructionVar173 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar174 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar175 {
    imm_00_07: TokenField_imm_00_07,
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
struct instructionVar176 {
    imm_00_07: TokenField_imm_00_07,
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
struct instructionVar177 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar178 {
    imm_00_07: TokenField_imm_00_07,
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
struct instructionVar179 {
    imm_00_07: TokenField_imm_00_07,
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
struct instructionVar180 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
struct instructionVar181 {
    imm_00_07: TokenField_imm_00_07,
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
struct instructionVar182 {
    imm_00_07: TokenField_imm_00_07,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1502:1"]
#[derive(Clone, Debug)]
struct instructionVar183 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("shad"),
            DisplayElement::Literal(" "),
            self.rm_04_07.display(),
            DisplayElement::Literal(", "),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1534:1"]
#[derive(Clone, Debug)]
struct instructionVar184 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("shld"),
            DisplayElement::Literal("  "),
            self.rm_04_07.display(),
            DisplayElement::Literal(", "),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1599:1"]
#[derive(Clone, Debug)]
struct instructionVar185 {
    target00_07: Tabletarget00_07,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1605:1"]
#[derive(Clone, Debug)]
struct instructionVar186 {
    target00_07: Tabletarget00_07,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("bf"),
            DisplayElement::Literal("/s"),
            DisplayElement::Literal(" "),
        ];
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
        if token_parser.TokenFieldopcode_08_15().disassembly() != 143i64 {
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
struct instructionVar187 {
    target00_07: Tabletarget00_07,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1619:1"]
#[derive(Clone, Debug)]
struct instructionVar188 {
    target00_07: Tabletarget00_07,
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
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("bt"),
            DisplayElement::Literal("/s"),
            DisplayElement::Literal(" "),
        ];
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
        if token_parser.TokenFieldopcode_08_15().disassembly() != 141i64 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1704:1"]
#[derive(Clone, Debug)]
struct instructionVar189 {
    disp_00_07: TokenField_disp_00_07,
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
        let mut calc_disp: i64 = 0;
        calc_disp = self.disp_00_07.disassembly().wrapping_mul(4i64);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("jsr"),
            DisplayElement::Literal("/n"),
            DisplayElement::Literal(" @@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(", "),
            DisplayElement::Register(Register::tbr),
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
        if token_parser.TokenFieldopcode_08_15().disassembly() != 131i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFielddisp_00_07()
            .disassembly()
            .wrapping_mul(4i64);
        let disp_00_07 = token_parser.TokenFielddisp_00_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:1980:1"]
#[derive(Clone, Debug)]
struct instructionVar190 {
    imm_00_07: TokenField_imm_00_07,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2007:1"]
#[derive(Clone, Debug)]
struct instructionVar191 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fadd"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 0i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2015:1"]
#[derive(Clone, Debug)]
struct instructionVar192 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("fcmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 4i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2023:1"]
#[derive(Clone, Debug)]
struct instructionVar193 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("fcmp"),
            DisplayElement::Literal("/gt"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 5i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2043:1"]
#[derive(Clone, Debug)]
struct instructionVar194 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fdiv"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 3i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2078:1"]
#[derive(Clone, Debug)]
struct instructionVar195 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("fmac"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::fr0),
            DisplayElement::Literal(", "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 14i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2084:1"]
#[derive(Clone, Debug)]
struct instructionVar196 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fmov"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 12i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2092:1"]
#[derive(Clone, Debug)]
struct instructionVar197 {
    f_rm_04_07: TokenField_f_rm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" @("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", "),
            self.f_rm_04_07.display(),
            DisplayElement::Literal("), "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 6i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let f_rm_04_07 = token_parser.TokenFieldf_rm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                f_rm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2100:1"]
#[derive(Clone, Debug)]
struct instructionVar198 {
    f_rm_04_07: TokenField_f_rm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" @"),
            self.f_rm_04_07.display(),
            DisplayElement::Literal("+, "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 9i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let f_rm_04_07 = token_parser.TokenFieldf_rm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                f_rm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2109:1"]
#[derive(Clone, Debug)]
struct instructionVar199 {
    f_rm_04_07: TokenField_f_rm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" @"),
            self.f_rm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 8i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let f_rm_04_07 = token_parser.TokenFieldf_rm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                f_rm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2127:1"]
#[derive(Clone, Debug)]
struct instructionVar200 {
    ffrm_04_07: TokenField_ffrm_04_07,
    f_rn_08_11: TokenField_f_rn_08_11,
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
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", @( "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", "),
            self.f_rn_08_11.display(),
            DisplayElement::Literal(" )"),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 7i64 {
            return None;
        }
        let f_rn_08_11 = token_parser.TokenFieldf_rn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                f_rn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2135:1"]
#[derive(Clone, Debug)]
struct instructionVar201 {
    ffrm_04_07: TokenField_ffrm_04_07,
    f_rn_08_11: TokenField_f_rn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", @-"),
            self.f_rn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 11i64 {
            return None;
        }
        let f_rn_08_11 = token_parser.TokenFieldf_rn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                f_rn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2144:1"]
#[derive(Clone, Debug)]
struct instructionVar202 {
    ffrm_04_07: TokenField_ffrm_04_07,
    f_rn_08_11: TokenField_f_rn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", @"),
            self.f_rn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 10i64 {
            return None;
        }
        let f_rn_08_11 = token_parser.TokenFieldf_rn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                f_rn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2163:1"]
#[derive(Clone, Debug)]
struct instructionVar203 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fmul"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 2i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2196:1"]
#[derive(Clone, Debug)]
struct instructionVar204 {
    ffrm_04_07: TokenField_ffrm_04_07,
    ffrn_08_11: TokenField_ffrn_08_11,
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fsub"),
            DisplayElement::Literal(" "),
            self.ffrm_04_07.display(),
            DisplayElement::Literal(", "),
            self.ffrn_08_11.display(),
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
        if token_parser.TokenFieldfop_12_15().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldfop_00_03().disassembly() != 1i64 {
            return None;
        }
        let ffrn_08_11 = token_parser.TokenFieldffrn_08_11();
        let ffrm_04_07 = token_parser.TokenFieldffrm_04_07();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrn_08_11,
                ffrm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:186:1"]
#[derive(Clone, Debug)]
struct instructionVar205 {
    rn_08_11: TokenField_rn_08_11,
    imm8: Tableimm8,
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
struct instructionVar206 {
    rn_08_11: TokenField_rn_08_11,
    disppc2: Tabledisppc2,
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
struct instructionVar207 {
    rn_08_11: TokenField_rn_08_11,
    disppc4: Tabledisppc4,
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
struct instructionVar208 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
    disp_00_03: TokenField_disp_00_03,
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
struct instructionVar209 {
    rm_04_07: TokenField_rm_04_07,
    rn_08_11: TokenField_rn_08_11,
    disp_00_03: TokenField_disp_00_03,
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2291:1"]
#[derive(Clone, Debug)]
struct instructionVar210 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
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
        let extend: [DisplayElement; 9usize] = [
            DisplayElement::Literal("band.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(" @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 4i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2299:1"]
#[derive(Clone, Debug)]
struct instructionVar211 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
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
        let extend: [DisplayElement; 9usize] = [
            DisplayElement::Literal("bandnot.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 12i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2307:1"]
#[derive(Clone, Debug)]
struct instructionVar212 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar212 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bclr.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 0i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2324:1"]
#[derive(Clone, Debug)]
struct instructionVar213 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar213 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bld.b"),
            DisplayElement::Literal("  "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 3i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2340:1"]
#[derive(Clone, Debug)]
struct instructionVar214 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar214 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bldnot.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 11i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2348:1"]
#[derive(Clone, Debug)]
struct instructionVar215 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar215 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bor.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 5i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2357:1"]
#[derive(Clone, Debug)]
struct instructionVar216 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar216 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bornot.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 13i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2366:1"]
#[derive(Clone, Debug)]
struct instructionVar217 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar217 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bset.b"),
            DisplayElement::Literal("     "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 1i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2382:1"]
#[derive(Clone, Debug)]
struct instructionVar218 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar218 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bst.b"),
            DisplayElement::Literal("      "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 2i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2400:1"]
#[derive(Clone, Debug)]
struct instructionVar219 {
    l_imm3_20_22: TokenField_l_imm3_20_22,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar219 {
    fn display_extend<T>(
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
            DisplayElement::Literal("bxor.b"),
            DisplayElement::Literal("     "),
            DisplayElement::Literal("#"),
            self.l_imm3_20_22.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_23_23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 6i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_imm3_20_22 = token_parser.TokenFieldl_imm3_20_22();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_imm3_20_22,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:444:1"]
#[derive(Clone, Debug)]
struct instructionVar220 {
    l_rm_20_23: TokenField_l_rm_20_23,
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar220 {
    fn display_extend<T>(
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
            self.l_rm_20_23.display(),
            DisplayElement::Literal(", @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 0i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:450:1"]
#[derive(Clone, Debug)]
struct instructionVar221 {
    l_rm_20_23: TokenField_l_rm_20_23,
    l_rn_24_27: TokenField_l_rn_24_27,
    l_disp_00_11: TokenField_l_disp_00_11,
}
impl instructionVar221 {
    fn display_extend<T>(
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
        calc_disp = 2i64.wrapping_mul(self.l_disp_00_11.disassembly());
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  "),
            self.l_rm_20_23.display(),
            DisplayElement::Literal(", @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 1i64 {
            return None;
        }
        calc_disp = 2i64
            .wrapping_mul(token_parser.TokenFieldl_disp_00_11().disassembly());
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:458:1"]
#[derive(Clone, Debug)]
struct instructionVar222 {
    l_rm_20_23: TokenField_l_rm_20_23,
    l_rn_24_27: TokenField_l_rn_24_27,
    l_disp_00_11: TokenField_l_disp_00_11,
}
impl instructionVar222 {
    fn display_extend<T>(
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
        calc_disp = 4i64.wrapping_mul(self.l_disp_00_11.disassembly());
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  "),
            self.l_rm_20_23.display(),
            DisplayElement::Literal(", @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(", "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 2i64 {
            return None;
        }
        calc_disp = 4i64
            .wrapping_mul(token_parser.TokenFieldl_disp_00_11().disassembly());
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:466:1"]
#[derive(Clone, Debug)]
struct instructionVar223 {
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rm_20_23: TokenField_l_rm_20_23,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar223 {
    fn display_extend<T>(
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
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rm_20_23.display(),
            DisplayElement::Literal("), "),
            self.l_rn_24_27.display(),
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
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 4i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:472:1"]
#[derive(Clone, Debug)]
struct instructionVar224 {
    l_rm_20_23: TokenField_l_rm_20_23,
    l_rn_24_27: TokenField_l_rn_24_27,
    l_disp_00_11: TokenField_l_disp_00_11,
}
impl instructionVar224 {
    fn display_extend<T>(
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
        calc_disp = 2i64.wrapping_mul(self.l_disp_00_11.disassembly());
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal("  @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(", "),
            self.l_rm_20_23.display(),
            DisplayElement::Literal("), "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 5i64 {
            return None;
        }
        calc_disp = 2i64
            .wrapping_mul(token_parser.TokenFieldl_disp_00_11().disassembly());
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:480:1"]
#[derive(Clone, Debug)]
struct instructionVar225 {
    l_rm_20_23: TokenField_l_rm_20_23,
    l_rn_24_27: TokenField_l_rn_24_27,
    l_disp_00_11: TokenField_l_disp_00_11,
}
impl instructionVar225 {
    fn display_extend<T>(
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
        calc_disp = 4i64.wrapping_mul(self.l_disp_00_11.disassembly());
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal("  @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(", "),
            self.l_rm_20_23.display(),
            DisplayElement::Literal("), "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 6i64 {
            return None;
        }
        calc_disp = 4i64
            .wrapping_mul(token_parser.TokenFieldl_disp_00_11().disassembly());
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:488:1"]
#[derive(Clone, Debug)]
struct instructionVar226 {
    l_disp_00_11: TokenField_l_disp_00_11,
    l_rm_20_23: TokenField_l_rm_20_23,
    l_rn_24_27: TokenField_l_rn_24_27,
}
impl instructionVar226 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movu.b"),
            DisplayElement::Literal(" @("),
            self.l_disp_00_11.display(),
            DisplayElement::Literal(", "),
            self.l_rm_20_23.display(),
            DisplayElement::Literal("), "),
            self.l_rn_24_27.display(),
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
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 8i64 {
            return None;
        }
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:495:1"]
#[derive(Clone, Debug)]
struct instructionVar227 {
    l_rm_20_23: TokenField_l_rm_20_23,
    l_rn_24_27: TokenField_l_rn_24_27,
    l_disp_00_11: TokenField_l_disp_00_11,
}
impl instructionVar227 {
    fn display_extend<T>(
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
        calc_disp = self.l_disp_00_11.disassembly().wrapping_mul(2i64);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("movu.w"),
            DisplayElement::Literal(" @("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(", "),
            self.l_rm_20_23.display(),
            DisplayElement::Literal("), "),
            self.l_rn_24_27.display(),
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
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_12_15().disassembly() != 9i64 {
            return None;
        }
        calc_disp = token_parser
            .TokenFieldl_disp_00_11()
            .disassembly()
            .wrapping_mul(2i64);
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        let l_rm_20_23 = token_parser.TokenFieldl_rm_20_23();
        let l_disp_00_11 = token_parser.TokenFieldl_disp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rn_24_27,
                l_rm_20_23,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2117:1"]
#[derive(Clone, Debug)]
struct instructionVar228 {
    lf_rm_20_23: TokenField_lf_rm_20_23,
    lffrn_24_27: TokenField_lffrn_24_27,
    lfdisp_00_11: TokenField_lfdisp_00_11,
}
impl instructionVar228 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp12: i64 = 0;
        calc_disp12 = self.lfdisp_00_11.disassembly().wrapping_mul(4i64);
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" @("),
            DisplayElement::Number(true, calc_disp12),
            DisplayElement::Literal(", "),
            self.lf_rm_20_23.display(),
            DisplayElement::Literal("), "),
            self.lffrn_24_27.display(),
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
        let mut calc_disp12: i64 = 0;
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldlfop_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldlfop_12_19().disassembly() != 23i64 {
            return None;
        }
        calc_disp12 = token_parser
            .TokenFieldlfdisp_00_11()
            .disassembly()
            .wrapping_mul(4i64);
        let lffrn_24_27 = token_parser.TokenFieldlffrn_24_27();
        let lf_rm_20_23 = token_parser.TokenFieldlf_rm_20_23();
        let lfdisp_00_11 = token_parser.TokenFieldlfdisp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                lffrn_24_27,
                lf_rm_20_23,
                lfdisp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:2152:1"]
#[derive(Clone, Debug)]
struct instructionVar229 {
    lffrm_20_23: TokenField_lffrm_20_23,
    lf_rn_24_27: TokenField_lf_rn_24_27,
    lfdisp_00_11: TokenField_lfdisp_00_11,
}
impl instructionVar229 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp12: i64 = 0;
        calc_disp12 = self.lfdisp_00_11.disassembly().wrapping_mul(4i64);
        let extend: [DisplayElement; 8usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            self.lffrm_20_23.display(),
            DisplayElement::Literal(", @("),
            DisplayElement::Number(true, calc_disp12),
            DisplayElement::Literal(", "),
            self.lf_rn_24_27.display(),
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
        let mut calc_disp12: i64 = 0;
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        if token_parser.TokenFieldlfop_28_31().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldlfop_12_19().disassembly() != 19i64 {
            return None;
        }
        calc_disp12 = token_parser
            .TokenFieldlfdisp_00_11()
            .disassembly()
            .wrapping_mul(4i64);
        let lf_rn_24_27 = token_parser.TokenFieldlf_rn_24_27();
        let lffrm_20_23 = token_parser.TokenFieldlffrm_20_23();
        let lfdisp_00_11 = token_parser.TokenFieldlfdisp_00_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                lf_rn_24_27,
                lffrm_20_23,
                lfdisp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:512:1"]
#[derive(Clone, Debug)]
struct instructionVar230 {
    l_rn_24_27: TokenField_l_rn_24_27,
    simm20: Tablesimm20,
}
impl instructionVar230 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movi20"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.simm20.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.l_rn_24_27.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 0i64 {
            return None;
        }
        let simm20 = if let Some((len, table)) = Tablesimm20::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm20, l_rn_24_27 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:519:1"]
#[derive(Clone, Debug)]
struct instructionVar231 {
    l_rn_24_27: TokenField_l_rn_24_27,
    simm20s: Tablesimm20s,
}
impl instructionVar231 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movi20s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.simm20s.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(", "), self.l_rn_24_27.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
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
        if token_parser.TokenFieldl_opcode_28_31().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldl_opcode_16_19().disassembly() != 1i64 {
            return None;
        }
        let simm20s = if let Some((len, table)) = Tablesimm20s::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let l_rn_24_27 = token_parser.TokenFieldl_rn_24_27();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                simm20s,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:868:1"]
#[derive(Clone, Debug)]
struct instructionVar232 {
    simm_00_07: TokenField_simm_00_07,
    rn_08_11: TokenField_rn_08_11,
}
impl instructionVar232 {
    fn display_extend<T>(
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
struct instructionVar233 {
    target00_11: Tabletarget00_11,
}
impl instructionVar233 {
    fn display_extend<T>(
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
struct instructionVar234 {
    target00_11: Tabletarget00_11,
}
impl instructionVar234 {
    fn display_extend<T>(
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
    Var212(instructionVar212),
    Var213(instructionVar213),
    Var214(instructionVar214),
    Var215(instructionVar215),
    Var216(instructionVar216),
    Var217(instructionVar217),
    Var218(instructionVar218),
    Var219(instructionVar219),
    Var220(instructionVar220),
    Var221(instructionVar221),
    Var222(instructionVar222),
    Var223(instructionVar223),
    Var224(instructionVar224),
    Var225(instructionVar225),
    Var226(instructionVar226),
    Var227(instructionVar227),
    Var228(instructionVar228),
    Var229(instructionVar229),
    Var230(instructionVar230),
    Var231(instructionVar231),
    Var232(instructionVar232),
    Var233(instructionVar233),
    Var234(instructionVar234),
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
            Self::Var216(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var217(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var218(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var219(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var220(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var221(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var222(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var223(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var224(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var225(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var226(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var227(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var228(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var229(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var230(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var231(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var232(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var233(x) => x.display_extend(
                display,
                context,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var234(x) => x.display_extend(
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
        if let Some((inst_len, parsed)) = instructionVar212::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var212(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar213::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var213(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar214::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var214(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar215::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var215(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar216::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var216(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar217::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var217(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar218::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var218(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar219::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var219(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar220::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var220(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar221::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var221(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar222::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var222(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar223::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var223(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar224::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var224(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar225::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var225(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar226::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var226(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar227::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var227(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar228::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var228(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar229::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var229(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar230::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var230(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar231::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var231(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar232::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var232(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar233::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var233(parsed)));
        }
        if let Some((inst_len, parsed)) = instructionVar234::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var234(parsed)));
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:502:1"]
#[derive(Clone, Debug)]
struct simm20Var0 {
    l_simm20_20_23: TokenField_l_simm20_20_23,
    l_imm20_00_15: TokenField_l_imm20_00_15,
}
impl simm20Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_value: i64 = 0;
        calc_value = (self
            .l_simm20_20_23
            .disassembly()
            .checked_shl(u32::try_from(16i64).unwrap())
            .unwrap_or(0)
            | self.l_imm20_00_15.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Number(true, calc_value),
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
        let mut calc_value: i64 = 0;
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        calc_value = (token_parser
            .TokenFieldl_simm20_20_23()
            .disassembly()
            .checked_shl(u32::try_from(16i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldl_imm20_00_15().disassembly());
        let l_simm20_20_23 = token_parser.TokenFieldl_simm20_20_23();
        let l_imm20_00_15 = token_parser.TokenFieldl_imm20_00_15();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_simm20_20_23,
                l_imm20_00_15,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tablesimm20 {
    Var0(simm20Var0),
}
impl Tablesimm20 {
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
            simm20Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:506:1"]
#[derive(Clone, Debug)]
struct simm20sVar0 {
    l_simm20_20_23: TokenField_l_simm20_20_23,
    l_imm20_00_15: TokenField_l_imm20_00_15,
}
impl simm20sVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_value: i64 = 0;
        calc_value = (self
            .l_simm20_20_23
            .disassembly()
            .checked_shl(u32::try_from(16i64).unwrap())
            .unwrap_or(0)
            | self.l_imm20_00_15.disassembly())
        .checked_shl(u32::try_from(8i64).unwrap())
        .unwrap_or(0);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Number(true, calc_value),
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
        let mut calc_value: i64 = 0;
        let mut block_0_len = 4u64 as u32;
        let token_parser = <TokenParser<4usize>>::new(tokens_current)?;
        calc_value = (token_parser
            .TokenFieldl_simm20_20_23()
            .disassembly()
            .checked_shl(u32::try_from(16i64).unwrap())
            .unwrap_or(0)
            | token_parser.TokenFieldl_imm20_00_15().disassembly())
        .checked_shl(u32::try_from(8i64).unwrap())
        .unwrap_or(0);
        let l_simm20_20_23 = token_parser.TokenFieldl_simm20_20_23();
        let l_imm20_00_15 = token_parser.TokenFieldl_imm20_00_15();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_simm20_20_23,
                l_imm20_00_15,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tablesimm20s {
    Var0(simm20sVar0),
}
impl Tablesimm20s {
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
            simm20sVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:539:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_0Var0 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMLReg1_0Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 0i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_0 {
    Var0(MovMLReg1_0Var0),
}
impl TableMovMLReg1_0 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:540:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_0storeVar0 {}
impl MovMLReg1_0storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_0store {
    Var0(MovMLReg1_0storeVar0),
}
impl TableMovMLReg1_0store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_0storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:542:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_1Var0 {
    MovMLReg1_0: TableMovMLReg1_0,
}
impl MovMLReg1_1Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_0 = if let Some((len, table)) = TableMovMLReg1_0::parse(
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
        Some((pattern_len, Self { MovMLReg1_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:543:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_1Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_0store: TableMovMLReg1_0store,
}
impl MovMLReg1_1Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 1i64 {
            return None;
        }
        let MovMLReg1_0store = if let Some((len, table)) =
            TableMovMLReg1_0store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_0store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_1 {
    Var0(MovMLReg1_1Var0),
    Var1(MovMLReg1_1Var1),
}
impl TableMovMLReg1_1 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_1Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_1Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:544:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_1storeVar0 {
    MovMLReg1_0store: TableMovMLReg1_0store,
}
impl MovMLReg1_1storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_0store = if let Some((len, table)) =
            TableMovMLReg1_0store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_0store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_1store {
    Var0(MovMLReg1_1storeVar0),
}
impl TableMovMLReg1_1store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_1storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:546:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_2Var0 {
    MovMLReg1_1: TableMovMLReg1_1,
}
impl MovMLReg1_2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_1 = if let Some((len, table)) = TableMovMLReg1_1::parse(
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
        Some((pattern_len, Self { MovMLReg1_1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:547:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_2Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_1store: TableMovMLReg1_1store,
}
impl MovMLReg1_2Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 2i64 {
            return None;
        }
        let MovMLReg1_1store = if let Some((len, table)) =
            TableMovMLReg1_1store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_1store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_2 {
    Var0(MovMLReg1_2Var0),
    Var1(MovMLReg1_2Var1),
}
impl TableMovMLReg1_2 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_2Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_2Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:548:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_2storeVar0 {
    MovMLReg1_1store: TableMovMLReg1_1store,
}
impl MovMLReg1_2storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_1store = if let Some((len, table)) =
            TableMovMLReg1_1store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_1store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_2store {
    Var0(MovMLReg1_2storeVar0),
}
impl TableMovMLReg1_2store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_2storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:550:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_3Var0 {
    MovMLReg1_2: TableMovMLReg1_2,
}
impl MovMLReg1_3Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_2 = if let Some((len, table)) = TableMovMLReg1_2::parse(
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
        Some((pattern_len, Self { MovMLReg1_2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:551:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_3Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_2store: TableMovMLReg1_2store,
}
impl MovMLReg1_3Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 3i64 {
            return None;
        }
        let MovMLReg1_2store = if let Some((len, table)) =
            TableMovMLReg1_2store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_2store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_3 {
    Var0(MovMLReg1_3Var0),
    Var1(MovMLReg1_3Var1),
}
impl TableMovMLReg1_3 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_3Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_3Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:552:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_3storeVar0 {
    MovMLReg1_2store: TableMovMLReg1_2store,
}
impl MovMLReg1_3storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_2store = if let Some((len, table)) =
            TableMovMLReg1_2store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_2store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_3store {
    Var0(MovMLReg1_3storeVar0),
}
impl TableMovMLReg1_3store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_3storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:554:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_4Var0 {
    MovMLReg1_3: TableMovMLReg1_3,
}
impl MovMLReg1_4Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_3.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_3 = if let Some((len, table)) = TableMovMLReg1_3::parse(
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
        Some((pattern_len, Self { MovMLReg1_3 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:555:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_4Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_3store: TableMovMLReg1_3store,
}
impl MovMLReg1_4Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 4i64 {
            return None;
        }
        let MovMLReg1_3store = if let Some((len, table)) =
            TableMovMLReg1_3store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_3store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_4 {
    Var0(MovMLReg1_4Var0),
    Var1(MovMLReg1_4Var1),
}
impl TableMovMLReg1_4 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_4Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_4Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:556:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_4storeVar0 {
    MovMLReg1_3store: TableMovMLReg1_3store,
}
impl MovMLReg1_4storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_3store = if let Some((len, table)) =
            TableMovMLReg1_3store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_3store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_4store {
    Var0(MovMLReg1_4storeVar0),
}
impl TableMovMLReg1_4store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_4storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:558:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_5Var0 {
    MovMLReg1_4: TableMovMLReg1_4,
}
impl MovMLReg1_5Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_4.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_4 = if let Some((len, table)) = TableMovMLReg1_4::parse(
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
        Some((pattern_len, Self { MovMLReg1_4 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:559:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_5Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_4store: TableMovMLReg1_4store,
}
impl MovMLReg1_5Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 5i64 {
            return None;
        }
        let MovMLReg1_4store = if let Some((len, table)) =
            TableMovMLReg1_4store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_4store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_5 {
    Var0(MovMLReg1_5Var0),
    Var1(MovMLReg1_5Var1),
}
impl TableMovMLReg1_5 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_5Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_5Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:560:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_5storeVar0 {
    MovMLReg1_4store: TableMovMLReg1_4store,
}
impl MovMLReg1_5storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_4store = if let Some((len, table)) =
            TableMovMLReg1_4store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_4store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_5store {
    Var0(MovMLReg1_5storeVar0),
}
impl TableMovMLReg1_5store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_5storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:562:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_6Var0 {
    MovMLReg1_5: TableMovMLReg1_5,
}
impl MovMLReg1_6Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_5.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_5 = if let Some((len, table)) = TableMovMLReg1_5::parse(
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
        Some((pattern_len, Self { MovMLReg1_5 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:563:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_6Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_5store: TableMovMLReg1_5store,
}
impl MovMLReg1_6Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 6i64 {
            return None;
        }
        let MovMLReg1_5store = if let Some((len, table)) =
            TableMovMLReg1_5store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_5store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_6 {
    Var0(MovMLReg1_6Var0),
    Var1(MovMLReg1_6Var1),
}
impl TableMovMLReg1_6 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_6Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_6Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:564:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_6storeVar0 {
    MovMLReg1_5store: TableMovMLReg1_5store,
}
impl MovMLReg1_6storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_5store = if let Some((len, table)) =
            TableMovMLReg1_5store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_5store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_6store {
    Var0(MovMLReg1_6storeVar0),
}
impl TableMovMLReg1_6store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_6storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:566:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_7Var0 {
    MovMLReg1_6: TableMovMLReg1_6,
}
impl MovMLReg1_7Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_6.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_6 = if let Some((len, table)) = TableMovMLReg1_6::parse(
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
        Some((pattern_len, Self { MovMLReg1_6 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:567:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_7Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_6store: TableMovMLReg1_6store,
}
impl MovMLReg1_7Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 7i64 {
            return None;
        }
        let MovMLReg1_6store = if let Some((len, table)) =
            TableMovMLReg1_6store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_6store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_7 {
    Var0(MovMLReg1_7Var0),
    Var1(MovMLReg1_7Var1),
}
impl TableMovMLReg1_7 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_7Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_7Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:568:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_7storeVar0 {
    MovMLReg1_6store: TableMovMLReg1_6store,
}
impl MovMLReg1_7storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_6store = if let Some((len, table)) =
            TableMovMLReg1_6store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_6store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_7store {
    Var0(MovMLReg1_7storeVar0),
}
impl TableMovMLReg1_7store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_7storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:570:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_8Var0 {
    MovMLReg1_7: TableMovMLReg1_7,
}
impl MovMLReg1_8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_7.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_7 = if let Some((len, table)) = TableMovMLReg1_7::parse(
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
        Some((pattern_len, Self { MovMLReg1_7 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:571:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_8Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_7store: TableMovMLReg1_7store,
}
impl MovMLReg1_8Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 8i64 {
            return None;
        }
        let MovMLReg1_7store = if let Some((len, table)) =
            TableMovMLReg1_7store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_7store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_8 {
    Var0(MovMLReg1_8Var0),
    Var1(MovMLReg1_8Var1),
}
impl TableMovMLReg1_8 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_8Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_8Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:572:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_8storeVar0 {
    MovMLReg1_7store: TableMovMLReg1_7store,
}
impl MovMLReg1_8storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_7store = if let Some((len, table)) =
            TableMovMLReg1_7store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_7store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_8store {
    Var0(MovMLReg1_8storeVar0),
}
impl TableMovMLReg1_8store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_8storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:574:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_9Var0 {
    MovMLReg1_8: TableMovMLReg1_8,
}
impl MovMLReg1_9Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_8 = if let Some((len, table)) = TableMovMLReg1_8::parse(
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
        Some((pattern_len, Self { MovMLReg1_8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:575:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_9Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_8store: TableMovMLReg1_8store,
}
impl MovMLReg1_9Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 9i64 {
            return None;
        }
        let MovMLReg1_8store = if let Some((len, table)) =
            TableMovMLReg1_8store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_8store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_9 {
    Var0(MovMLReg1_9Var0),
    Var1(MovMLReg1_9Var1),
}
impl TableMovMLReg1_9 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_9Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_9Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:576:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_9storeVar0 {
    MovMLReg1_8store: TableMovMLReg1_8store,
}
impl MovMLReg1_9storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_8store = if let Some((len, table)) =
            TableMovMLReg1_8store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_8store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_9store {
    Var0(MovMLReg1_9storeVar0),
}
impl TableMovMLReg1_9store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_9storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:578:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_10Var0 {
    MovMLReg1_9: TableMovMLReg1_9,
}
impl MovMLReg1_10Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_9.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_9 = if let Some((len, table)) = TableMovMLReg1_9::parse(
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
        Some((pattern_len, Self { MovMLReg1_9 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:579:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_10Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_9store: TableMovMLReg1_9store,
}
impl MovMLReg1_10Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 10i64 {
            return None;
        }
        let MovMLReg1_9store = if let Some((len, table)) =
            TableMovMLReg1_9store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_9store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_10 {
    Var0(MovMLReg1_10Var0),
    Var1(MovMLReg1_10Var1),
}
impl TableMovMLReg1_10 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_10Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_10Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:580:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_10storeVar0 {
    MovMLReg1_9store: TableMovMLReg1_9store,
}
impl MovMLReg1_10storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_9store = if let Some((len, table)) =
            TableMovMLReg1_9store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_9store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_10store {
    Var0(MovMLReg1_10storeVar0),
}
impl TableMovMLReg1_10store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_10storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:582:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_11Var0 {
    MovMLReg1_10: TableMovMLReg1_10,
}
impl MovMLReg1_11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_10.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_10 = if let Some((len, table)) = TableMovMLReg1_10::parse(
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
        Some((pattern_len, Self { MovMLReg1_10 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:583:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_11Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_10store: TableMovMLReg1_10store,
}
impl MovMLReg1_11Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 11i64 {
            return None;
        }
        let MovMLReg1_10store = if let Some((len, table)) =
            TableMovMLReg1_10store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_10store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_11 {
    Var0(MovMLReg1_11Var0),
    Var1(MovMLReg1_11Var1),
}
impl TableMovMLReg1_11 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_11Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_11Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:584:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_11storeVar0 {
    MovMLReg1_10store: TableMovMLReg1_10store,
}
impl MovMLReg1_11storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_10store = if let Some((len, table)) =
            TableMovMLReg1_10store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_10store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_11store {
    Var0(MovMLReg1_11storeVar0),
}
impl TableMovMLReg1_11store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_11storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:586:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_12Var0 {
    MovMLReg1_11: TableMovMLReg1_11,
}
impl MovMLReg1_12Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_11 = if let Some((len, table)) = TableMovMLReg1_11::parse(
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
        Some((pattern_len, Self { MovMLReg1_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:587:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_12Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_11store: TableMovMLReg1_11store,
}
impl MovMLReg1_12Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 12i64 {
            return None;
        }
        let MovMLReg1_11store = if let Some((len, table)) =
            TableMovMLReg1_11store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_11store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_12 {
    Var0(MovMLReg1_12Var0),
    Var1(MovMLReg1_12Var1),
}
impl TableMovMLReg1_12 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_12Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_12Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:588:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_12storeVar0 {
    MovMLReg1_11store: TableMovMLReg1_11store,
}
impl MovMLReg1_12storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_11store = if let Some((len, table)) =
            TableMovMLReg1_11store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_11store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_12store {
    Var0(MovMLReg1_12storeVar0),
}
impl TableMovMLReg1_12store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_12storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:590:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_13Var0 {
    MovMLReg1_12: TableMovMLReg1_12,
}
impl MovMLReg1_13Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_12.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_12 = if let Some((len, table)) = TableMovMLReg1_12::parse(
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
        Some((pattern_len, Self { MovMLReg1_12 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:591:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_13Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_12store: TableMovMLReg1_12store,
}
impl MovMLReg1_13Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 13i64 {
            return None;
        }
        let MovMLReg1_12store = if let Some((len, table)) =
            TableMovMLReg1_12store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_12store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_13 {
    Var0(MovMLReg1_13Var0),
    Var1(MovMLReg1_13Var1),
}
impl TableMovMLReg1_13 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_13Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_13Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:592:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_13storeVar0 {
    MovMLReg1_12store: TableMovMLReg1_12store,
}
impl MovMLReg1_13storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_12store = if let Some((len, table)) =
            TableMovMLReg1_12store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_12store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_13store {
    Var0(MovMLReg1_13storeVar0),
}
impl TableMovMLReg1_13store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_13storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:594:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_14Var0 {
    MovMLReg1_13: TableMovMLReg1_13,
}
impl MovMLReg1_14Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_13.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_13 = if let Some((len, table)) = TableMovMLReg1_13::parse(
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
        Some((pattern_len, Self { MovMLReg1_13 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:595:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_14Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_13store: TableMovMLReg1_13store,
}
impl MovMLReg1_14Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 14i64 {
            return None;
        }
        let MovMLReg1_13store = if let Some((len, table)) =
            TableMovMLReg1_13store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_13store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_14 {
    Var0(MovMLReg1_14Var0),
    Var1(MovMLReg1_14Var1),
}
impl TableMovMLReg1_14 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_14Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_14Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:596:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_14storeVar0 {
    MovMLReg1_13store: TableMovMLReg1_13store,
}
impl MovMLReg1_14storeVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg1_13store = if let Some((len, table)) =
            TableMovMLReg1_13store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_13store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_14store {
    Var0(MovMLReg1_14storeVar0),
}
impl TableMovMLReg1_14store {
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
        if let Some((inst_len, parsed)) = MovMLReg1_14storeVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:598:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_15Var0 {
    MovMLReg1_14: TableMovMLReg1_14,
}
impl MovMLReg1_15Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_14.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_14 = if let Some((len, table)) = TableMovMLReg1_14::parse(
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
        Some((pattern_len, Self { MovMLReg1_14 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:599:1"]
#[derive(Clone, Debug)]
struct MovMLReg1_15Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg1_14store: TableMovMLReg1_14store,
}
impl MovMLReg1_15Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 15i64 {
            return None;
        }
        let MovMLReg1_14store = if let Some((len, table)) =
            TableMovMLReg1_14store::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_14store,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_15 {
    Var0(MovMLReg1_15Var0),
    Var1(MovMLReg1_15Var1),
}
impl TableMovMLReg1_15 {
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
        if let Some((inst_len, parsed)) = MovMLReg1_15Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg1_15Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:601:1"]
#[derive(Clone, Debug)]
struct MovMLReg1Var0 {
    MovMLReg1_15: TableMovMLReg1_15,
}
impl MovMLReg1Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg1_15.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg1_15 = if let Some((len, table)) = TableMovMLReg1_15::parse(
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
        Some((pattern_len, Self { MovMLReg1_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1 {
    Var0(MovMLReg1Var0),
}
impl TableMovMLReg1 {
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
            MovMLReg1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:613:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_0Var0 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMLReg2_0Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 0i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_0 {
    Var0(MovMLReg2_0Var0),
}
impl TableMovMLReg2_0 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:615:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_0loadVar0 {}
impl MovMLReg2_0loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_0load {
    Var0(MovMLReg2_0loadVar0),
}
impl TableMovMLReg2_0load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_0loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:617:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_1Var0 {
    MovMLReg2_0: TableMovMLReg2_0,
}
impl MovMLReg2_1Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_0 = if let Some((len, table)) = TableMovMLReg2_0::parse(
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
        Some((pattern_len, Self { MovMLReg2_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:618:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_1Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_0load: TableMovMLReg2_0load,
}
impl MovMLReg2_1Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 1i64 {
            return None;
        }
        let MovMLReg2_0load = if let Some((len, table)) =
            TableMovMLReg2_0load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_0load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_1 {
    Var0(MovMLReg2_1Var0),
    Var1(MovMLReg2_1Var1),
}
impl TableMovMLReg2_1 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_1Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_1Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:619:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_1loadVar0 {
    MovMLReg2_0load: TableMovMLReg2_0load,
}
impl MovMLReg2_1loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_0load = if let Some((len, table)) =
            TableMovMLReg2_0load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_0load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_1load {
    Var0(MovMLReg2_1loadVar0),
}
impl TableMovMLReg2_1load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_1loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:621:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_2Var0 {
    MovMLReg2_1: TableMovMLReg2_1,
}
impl MovMLReg2_2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_1 = if let Some((len, table)) = TableMovMLReg2_1::parse(
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
        Some((pattern_len, Self { MovMLReg2_1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:622:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_2Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_1load: TableMovMLReg2_1load,
}
impl MovMLReg2_2Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 2i64 {
            return None;
        }
        let MovMLReg2_1load = if let Some((len, table)) =
            TableMovMLReg2_1load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_1load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_2 {
    Var0(MovMLReg2_2Var0),
    Var1(MovMLReg2_2Var1),
}
impl TableMovMLReg2_2 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_2Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_2Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:623:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_2loadVar0 {
    MovMLReg2_1load: TableMovMLReg2_1load,
}
impl MovMLReg2_2loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_1load = if let Some((len, table)) =
            TableMovMLReg2_1load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_1load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_2load {
    Var0(MovMLReg2_2loadVar0),
}
impl TableMovMLReg2_2load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_2loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:625:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_3Var0 {
    MovMLReg2_2: TableMovMLReg2_2,
}
impl MovMLReg2_3Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_2 = if let Some((len, table)) = TableMovMLReg2_2::parse(
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
        Some((pattern_len, Self { MovMLReg2_2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:626:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_3Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_2load: TableMovMLReg2_2load,
}
impl MovMLReg2_3Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 3i64 {
            return None;
        }
        let MovMLReg2_2load = if let Some((len, table)) =
            TableMovMLReg2_2load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_2load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_3 {
    Var0(MovMLReg2_3Var0),
    Var1(MovMLReg2_3Var1),
}
impl TableMovMLReg2_3 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_3Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_3Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:627:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_3loadVar0 {
    MovMLReg2_2load: TableMovMLReg2_2load,
}
impl MovMLReg2_3loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_2load = if let Some((len, table)) =
            TableMovMLReg2_2load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_2load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_3load {
    Var0(MovMLReg2_3loadVar0),
}
impl TableMovMLReg2_3load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_3loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:629:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_4Var0 {
    MovMLReg2_3: TableMovMLReg2_3,
}
impl MovMLReg2_4Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_3.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_3 = if let Some((len, table)) = TableMovMLReg2_3::parse(
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
        Some((pattern_len, Self { MovMLReg2_3 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:630:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_4Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_3load: TableMovMLReg2_3load,
}
impl MovMLReg2_4Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 4i64 {
            return None;
        }
        let MovMLReg2_3load = if let Some((len, table)) =
            TableMovMLReg2_3load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_3load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_4 {
    Var0(MovMLReg2_4Var0),
    Var1(MovMLReg2_4Var1),
}
impl TableMovMLReg2_4 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_4Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_4Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:632:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_4loadVar0 {
    MovMLReg2_3load: TableMovMLReg2_3load,
}
impl MovMLReg2_4loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_3load = if let Some((len, table)) =
            TableMovMLReg2_3load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_3load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_4load {
    Var0(MovMLReg2_4loadVar0),
}
impl TableMovMLReg2_4load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_4loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:634:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_5Var0 {
    MovMLReg2_4: TableMovMLReg2_4,
}
impl MovMLReg2_5Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_4.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_4 = if let Some((len, table)) = TableMovMLReg2_4::parse(
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
        Some((pattern_len, Self { MovMLReg2_4 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:635:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_5Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_4load: TableMovMLReg2_4load,
}
impl MovMLReg2_5Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 5i64 {
            return None;
        }
        let MovMLReg2_4load = if let Some((len, table)) =
            TableMovMLReg2_4load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_4load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_5 {
    Var0(MovMLReg2_5Var0),
    Var1(MovMLReg2_5Var1),
}
impl TableMovMLReg2_5 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_5Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_5Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:637:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_5loadVar0 {
    MovMLReg2_4load: TableMovMLReg2_4load,
}
impl MovMLReg2_5loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_4load = if let Some((len, table)) =
            TableMovMLReg2_4load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_4load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_5load {
    Var0(MovMLReg2_5loadVar0),
}
impl TableMovMLReg2_5load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_5loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:639:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_6Var0 {
    MovMLReg2_5: TableMovMLReg2_5,
}
impl MovMLReg2_6Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_5.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_5 = if let Some((len, table)) = TableMovMLReg2_5::parse(
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
        Some((pattern_len, Self { MovMLReg2_5 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:640:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_6Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_5load: TableMovMLReg2_5load,
}
impl MovMLReg2_6Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 6i64 {
            return None;
        }
        let MovMLReg2_5load = if let Some((len, table)) =
            TableMovMLReg2_5load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_5load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_6 {
    Var0(MovMLReg2_6Var0),
    Var1(MovMLReg2_6Var1),
}
impl TableMovMLReg2_6 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_6Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_6Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:642:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_6loadVar0 {
    MovMLReg2_5load: TableMovMLReg2_5load,
}
impl MovMLReg2_6loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_5load = if let Some((len, table)) =
            TableMovMLReg2_5load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_5load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_6load {
    Var0(MovMLReg2_6loadVar0),
}
impl TableMovMLReg2_6load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_6loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:644:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_7Var0 {
    MovMLReg2_6: TableMovMLReg2_6,
}
impl MovMLReg2_7Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_6.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_6 = if let Some((len, table)) = TableMovMLReg2_6::parse(
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
        Some((pattern_len, Self { MovMLReg2_6 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:645:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_7Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_6load: TableMovMLReg2_6load,
}
impl MovMLReg2_7Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 7i64 {
            return None;
        }
        let MovMLReg2_6load = if let Some((len, table)) =
            TableMovMLReg2_6load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_6load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_7 {
    Var0(MovMLReg2_7Var0),
    Var1(MovMLReg2_7Var1),
}
impl TableMovMLReg2_7 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_7Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_7Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:647:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_7loadVar0 {
    MovMLReg2_6load: TableMovMLReg2_6load,
}
impl MovMLReg2_7loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_6load = if let Some((len, table)) =
            TableMovMLReg2_6load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_6load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_7load {
    Var0(MovMLReg2_7loadVar0),
}
impl TableMovMLReg2_7load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_7loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:649:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_8Var0 {
    MovMLReg2_7: TableMovMLReg2_7,
}
impl MovMLReg2_8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_7.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_7 = if let Some((len, table)) = TableMovMLReg2_7::parse(
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
        Some((pattern_len, Self { MovMLReg2_7 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:650:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_8Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_7load: TableMovMLReg2_7load,
}
impl MovMLReg2_8Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 8i64 {
            return None;
        }
        let MovMLReg2_7load = if let Some((len, table)) =
            TableMovMLReg2_7load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_7load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_8 {
    Var0(MovMLReg2_8Var0),
    Var1(MovMLReg2_8Var1),
}
impl TableMovMLReg2_8 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_8Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_8Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:652:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_8loadVar0 {
    MovMLReg2_7load: TableMovMLReg2_7load,
}
impl MovMLReg2_8loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_7load = if let Some((len, table)) =
            TableMovMLReg2_7load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_7load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_8load {
    Var0(MovMLReg2_8loadVar0),
}
impl TableMovMLReg2_8load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_8loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:654:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_9Var0 {
    MovMLReg2_8: TableMovMLReg2_8,
}
impl MovMLReg2_9Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_8 = if let Some((len, table)) = TableMovMLReg2_8::parse(
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
        Some((pattern_len, Self { MovMLReg2_8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:655:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_9Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_8load: TableMovMLReg2_8load,
}
impl MovMLReg2_9Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 9i64 {
            return None;
        }
        let MovMLReg2_8load = if let Some((len, table)) =
            TableMovMLReg2_8load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_8load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_9 {
    Var0(MovMLReg2_9Var0),
    Var1(MovMLReg2_9Var1),
}
impl TableMovMLReg2_9 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_9Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_9Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:657:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_9loadVar0 {
    MovMLReg2_8load: TableMovMLReg2_8load,
}
impl MovMLReg2_9loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_8load = if let Some((len, table)) =
            TableMovMLReg2_8load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_8load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_9load {
    Var0(MovMLReg2_9loadVar0),
}
impl TableMovMLReg2_9load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_9loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:659:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_10Var0 {
    MovMLReg2_9: TableMovMLReg2_9,
}
impl MovMLReg2_10Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_9.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_9 = if let Some((len, table)) = TableMovMLReg2_9::parse(
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
        Some((pattern_len, Self { MovMLReg2_9 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:660:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_10Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_9load: TableMovMLReg2_9load,
}
impl MovMLReg2_10Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 10i64 {
            return None;
        }
        let MovMLReg2_9load = if let Some((len, table)) =
            TableMovMLReg2_9load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_9load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_10 {
    Var0(MovMLReg2_10Var0),
    Var1(MovMLReg2_10Var1),
}
impl TableMovMLReg2_10 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_10Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_10Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:662:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_10loadVar0 {
    MovMLReg2_9load: TableMovMLReg2_9load,
}
impl MovMLReg2_10loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_9load = if let Some((len, table)) =
            TableMovMLReg2_9load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_9load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_10load {
    Var0(MovMLReg2_10loadVar0),
}
impl TableMovMLReg2_10load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_10loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:664:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_11Var0 {
    MovMLReg2_10: TableMovMLReg2_10,
}
impl MovMLReg2_11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_10.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_10 = if let Some((len, table)) = TableMovMLReg2_10::parse(
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
        Some((pattern_len, Self { MovMLReg2_10 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:665:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_11Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_10load: TableMovMLReg2_10load,
}
impl MovMLReg2_11Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 11i64 {
            return None;
        }
        let MovMLReg2_10load = if let Some((len, table)) =
            TableMovMLReg2_10load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_10load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_11 {
    Var0(MovMLReg2_11Var0),
    Var1(MovMLReg2_11Var1),
}
impl TableMovMLReg2_11 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_11Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_11Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:666:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_11loadVar0 {
    MovMLReg2_10load: TableMovMLReg2_10load,
}
impl MovMLReg2_11loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_10load = if let Some((len, table)) =
            TableMovMLReg2_10load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_10load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_11load {
    Var0(MovMLReg2_11loadVar0),
}
impl TableMovMLReg2_11load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_11loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:668:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_12Var0 {
    MovMLReg2_11: TableMovMLReg2_11,
}
impl MovMLReg2_12Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_11 = if let Some((len, table)) = TableMovMLReg2_11::parse(
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
        Some((pattern_len, Self { MovMLReg2_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:669:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_12Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_11load: TableMovMLReg2_11load,
}
impl MovMLReg2_12Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 12i64 {
            return None;
        }
        let MovMLReg2_11load = if let Some((len, table)) =
            TableMovMLReg2_11load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_11load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_12 {
    Var0(MovMLReg2_12Var0),
    Var1(MovMLReg2_12Var1),
}
impl TableMovMLReg2_12 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_12Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_12Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:671:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_12loadVar0 {
    MovMLReg2_11load: TableMovMLReg2_11load,
}
impl MovMLReg2_12loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_11load = if let Some((len, table)) =
            TableMovMLReg2_11load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_11load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_12load {
    Var0(MovMLReg2_12loadVar0),
}
impl TableMovMLReg2_12load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_12loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:673:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_13Var0 {
    MovMLReg2_12: TableMovMLReg2_12,
}
impl MovMLReg2_13Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_12.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_12 = if let Some((len, table)) = TableMovMLReg2_12::parse(
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
        Some((pattern_len, Self { MovMLReg2_12 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:674:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_13Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_12load: TableMovMLReg2_12load,
}
impl MovMLReg2_13Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 13i64 {
            return None;
        }
        let MovMLReg2_12load = if let Some((len, table)) =
            TableMovMLReg2_12load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_12load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_13 {
    Var0(MovMLReg2_13Var0),
    Var1(MovMLReg2_13Var1),
}
impl TableMovMLReg2_13 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_13Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_13Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:676:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_13loadVar0 {
    MovMLReg2_12load: TableMovMLReg2_12load,
}
impl MovMLReg2_13loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_12load = if let Some((len, table)) =
            TableMovMLReg2_12load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_12load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_13load {
    Var0(MovMLReg2_13loadVar0),
}
impl TableMovMLReg2_13load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_13loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:678:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_14Var0 {
    MovMLReg2_13: TableMovMLReg2_13,
}
impl MovMLReg2_14Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_13.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_13 = if let Some((len, table)) = TableMovMLReg2_13::parse(
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
        Some((pattern_len, Self { MovMLReg2_13 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:679:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_14Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_13load: TableMovMLReg2_13load,
}
impl MovMLReg2_14Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 14i64 {
            return None;
        }
        let MovMLReg2_13load = if let Some((len, table)) =
            TableMovMLReg2_13load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_13load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_14 {
    Var0(MovMLReg2_14Var0),
    Var1(MovMLReg2_14Var1),
}
impl TableMovMLReg2_14 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_14Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_14Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:681:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_14loadVar0 {
    MovMLReg2_13load: TableMovMLReg2_13load,
}
impl MovMLReg2_14loadVar0 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let MovMLReg2_13load = if let Some((len, table)) =
            TableMovMLReg2_13load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_13load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_14load {
    Var0(MovMLReg2_14loadVar0),
}
impl TableMovMLReg2_14load {
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
        if let Some((inst_len, parsed)) = MovMLReg2_14loadVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:683:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_15Var0 {
    MovMLReg2_14: TableMovMLReg2_14,
}
impl MovMLReg2_15Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_14.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_14 = if let Some((len, table)) = TableMovMLReg2_14::parse(
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
        Some((pattern_len, Self { MovMLReg2_14 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:684:1"]
#[derive(Clone, Debug)]
struct MovMLReg2_15Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
    MovMLReg2_14load: TableMovMLReg2_14load,
}
impl MovMLReg2_15Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 15i64 {
            return None;
        }
        let MovMLReg2_14load = if let Some((len, table)) =
            TableMovMLReg2_14load::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_14load,
                rm_imm_08_11,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_15 {
    Var0(MovMLReg2_15Var0),
    Var1(MovMLReg2_15Var1),
}
impl TableMovMLReg2_15 {
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
        if let Some((inst_len, parsed)) = MovMLReg2_15Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMLReg2_15Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:686:1"]
#[derive(Clone, Debug)]
struct MovMLReg2Var0 {
    MovMLReg2_15: TableMovMLReg2_15,
}
impl MovMLReg2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMLReg2_15.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMLReg2_15 = if let Some((len, table)) = TableMovMLReg2_15::parse(
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
        Some((pattern_len, Self { MovMLReg2_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2 {
    Var0(MovMLReg2Var0),
}
impl TableMovMLReg2 {
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
            MovMLReg2Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:697:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_0Var0 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_0Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 0i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_0 {
    Var0(MovMUReg1_0Var0),
}
impl TableMovMUReg1_0 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:699:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_1Var0 {
    MovMUReg1_0: TableMovMUReg1_0,
}
impl MovMUReg1_1Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_0 = if let Some((len, table)) = TableMovMUReg1_0::parse(
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
        Some((pattern_len, Self { MovMUReg1_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:700:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_1Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_1Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 1i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_1 {
    Var0(MovMUReg1_1Var0),
    Var1(MovMUReg1_1Var1),
}
impl TableMovMUReg1_1 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_1Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_1Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:702:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_2Var0 {
    MovMUReg1_1: TableMovMUReg1_1,
}
impl MovMUReg1_2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_1 = if let Some((len, table)) = TableMovMUReg1_1::parse(
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
        Some((pattern_len, Self { MovMUReg1_1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:703:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_2Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_2Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 2i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_2 {
    Var0(MovMUReg1_2Var0),
    Var1(MovMUReg1_2Var1),
}
impl TableMovMUReg1_2 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_2Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_2Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:705:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_3Var0 {
    MovMUReg1_2: TableMovMUReg1_2,
}
impl MovMUReg1_3Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_2 = if let Some((len, table)) = TableMovMUReg1_2::parse(
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
        Some((pattern_len, Self { MovMUReg1_2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:706:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_3Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_3Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 3i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_3 {
    Var0(MovMUReg1_3Var0),
    Var1(MovMUReg1_3Var1),
}
impl TableMovMUReg1_3 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_3Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_3Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:708:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_4Var0 {
    MovMUReg1_3: TableMovMUReg1_3,
}
impl MovMUReg1_4Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_3.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_3 = if let Some((len, table)) = TableMovMUReg1_3::parse(
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
        Some((pattern_len, Self { MovMUReg1_3 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:709:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_4Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_4Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 4i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_4 {
    Var0(MovMUReg1_4Var0),
    Var1(MovMUReg1_4Var1),
}
impl TableMovMUReg1_4 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_4Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_4Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:711:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_5Var0 {
    MovMUReg1_4: TableMovMUReg1_4,
}
impl MovMUReg1_5Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_4.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_4 = if let Some((len, table)) = TableMovMUReg1_4::parse(
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
        Some((pattern_len, Self { MovMUReg1_4 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:712:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_5Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_5Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 5i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_5 {
    Var0(MovMUReg1_5Var0),
    Var1(MovMUReg1_5Var1),
}
impl TableMovMUReg1_5 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_5Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_5Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:714:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_6Var0 {
    MovMUReg1_5: TableMovMUReg1_5,
}
impl MovMUReg1_6Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_5.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_5 = if let Some((len, table)) = TableMovMUReg1_5::parse(
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
        Some((pattern_len, Self { MovMUReg1_5 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:715:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_6Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_6Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 6i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_6 {
    Var0(MovMUReg1_6Var0),
    Var1(MovMUReg1_6Var1),
}
impl TableMovMUReg1_6 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_6Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_6Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:717:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_7Var0 {
    MovMUReg1_6: TableMovMUReg1_6,
}
impl MovMUReg1_7Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_6.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_6 = if let Some((len, table)) = TableMovMUReg1_6::parse(
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
        Some((pattern_len, Self { MovMUReg1_6 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:718:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_7Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_7Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 7i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_7 {
    Var0(MovMUReg1_7Var0),
    Var1(MovMUReg1_7Var1),
}
impl TableMovMUReg1_7 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_7Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_7Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:720:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_8Var0 {
    MovMUReg1_7: TableMovMUReg1_7,
}
impl MovMUReg1_8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_7.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_7 = if let Some((len, table)) = TableMovMUReg1_7::parse(
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
        Some((pattern_len, Self { MovMUReg1_7 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:721:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_8Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_8Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 8i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_8 {
    Var0(MovMUReg1_8Var0),
    Var1(MovMUReg1_8Var1),
}
impl TableMovMUReg1_8 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_8Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_8Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:723:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_9Var0 {
    MovMUReg1_8: TableMovMUReg1_8,
}
impl MovMUReg1_9Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_8 = if let Some((len, table)) = TableMovMUReg1_8::parse(
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
        Some((pattern_len, Self { MovMUReg1_8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:724:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_9Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_9Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 9i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_9 {
    Var0(MovMUReg1_9Var0),
    Var1(MovMUReg1_9Var1),
}
impl TableMovMUReg1_9 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_9Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_9Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:726:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_10Var0 {
    MovMUReg1_9: TableMovMUReg1_9,
}
impl MovMUReg1_10Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_9.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_9 = if let Some((len, table)) = TableMovMUReg1_9::parse(
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
        Some((pattern_len, Self { MovMUReg1_9 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:727:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_10Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_10Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 10i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_10 {
    Var0(MovMUReg1_10Var0),
    Var1(MovMUReg1_10Var1),
}
impl TableMovMUReg1_10 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_10Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_10Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:729:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_11Var0 {
    MovMUReg1_10: TableMovMUReg1_10,
}
impl MovMUReg1_11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_10.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_10 = if let Some((len, table)) = TableMovMUReg1_10::parse(
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
        Some((pattern_len, Self { MovMUReg1_10 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:730:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_11Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_11Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 11i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_11 {
    Var0(MovMUReg1_11Var0),
    Var1(MovMUReg1_11Var1),
}
impl TableMovMUReg1_11 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_11Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_11Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:732:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_12Var0 {
    MovMUReg1_11: TableMovMUReg1_11,
}
impl MovMUReg1_12Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_11 = if let Some((len, table)) = TableMovMUReg1_11::parse(
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
        Some((pattern_len, Self { MovMUReg1_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:733:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_12Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_12Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 12i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_12 {
    Var0(MovMUReg1_12Var0),
    Var1(MovMUReg1_12Var1),
}
impl TableMovMUReg1_12 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_12Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_12Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:735:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_13Var0 {
    MovMUReg1_12: TableMovMUReg1_12,
}
impl MovMUReg1_13Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_12.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_12 = if let Some((len, table)) = TableMovMUReg1_12::parse(
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
        Some((pattern_len, Self { MovMUReg1_12 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:736:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_13Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_13Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 13i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_13 {
    Var0(MovMUReg1_13Var0),
    Var1(MovMUReg1_13Var1),
}
impl TableMovMUReg1_13 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_13Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_13Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:738:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_14Var0 {
    MovMUReg1_13: TableMovMUReg1_13,
}
impl MovMUReg1_14Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_13.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_13 = if let Some((len, table)) = TableMovMUReg1_13::parse(
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
        Some((pattern_len, Self { MovMUReg1_13 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:739:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_14Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_14Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 14i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_14 {
    Var0(MovMUReg1_14Var0),
    Var1(MovMUReg1_14Var1),
}
impl TableMovMUReg1_14 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_14Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_14Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:741:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_15Var0 {
    MovMUReg1_14: TableMovMUReg1_14,
}
impl MovMUReg1_15Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_14.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_14 = if let Some((len, table)) = TableMovMUReg1_14::parse(
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
        Some((pattern_len, Self { MovMUReg1_14 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:742:1"]
#[derive(Clone, Debug)]
struct MovMUReg1_15Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg1_15Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 15i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_15 {
    Var0(MovMUReg1_15Var0),
    Var1(MovMUReg1_15Var1),
}
impl TableMovMUReg1_15 {
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
        if let Some((inst_len, parsed)) = MovMUReg1_15Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg1_15Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:744:1"]
#[derive(Clone, Debug)]
struct MovMUReg1Var0 {
    MovMUReg1_15: TableMovMUReg1_15,
}
impl MovMUReg1Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg1_15.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg1_15 = if let Some((len, table)) = TableMovMUReg1_15::parse(
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
        Some((pattern_len, Self { MovMUReg1_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1 {
    Var0(MovMUReg1Var0),
}
impl TableMovMUReg1 {
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
            MovMUReg1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:755:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_0Var0 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_0Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 0i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_0 {
    Var0(MovMUReg2_0Var0),
}
impl TableMovMUReg2_0 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:757:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_1Var0 {
    MovMUReg2_0: TableMovMUReg2_0,
}
impl MovMUReg2_1Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_0 = if let Some((len, table)) = TableMovMUReg2_0::parse(
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
        Some((pattern_len, Self { MovMUReg2_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:758:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_1Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_1Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 1i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_1 {
    Var0(MovMUReg2_1Var0),
    Var1(MovMUReg2_1Var1),
}
impl TableMovMUReg2_1 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_1Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_1Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:760:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_2Var0 {
    MovMUReg2_1: TableMovMUReg2_1,
}
impl MovMUReg2_2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_1 = if let Some((len, table)) = TableMovMUReg2_1::parse(
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
        Some((pattern_len, Self { MovMUReg2_1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:761:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_2Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_2Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 2i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_2 {
    Var0(MovMUReg2_2Var0),
    Var1(MovMUReg2_2Var1),
}
impl TableMovMUReg2_2 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_2Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_2Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:763:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_3Var0 {
    MovMUReg2_2: TableMovMUReg2_2,
}
impl MovMUReg2_3Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_2 = if let Some((len, table)) = TableMovMUReg2_2::parse(
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
        Some((pattern_len, Self { MovMUReg2_2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:764:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_3Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_3Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 3i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_3 {
    Var0(MovMUReg2_3Var0),
    Var1(MovMUReg2_3Var1),
}
impl TableMovMUReg2_3 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_3Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_3Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:766:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_4Var0 {
    MovMUReg2_3: TableMovMUReg2_3,
}
impl MovMUReg2_4Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_3.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_3 = if let Some((len, table)) = TableMovMUReg2_3::parse(
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
        Some((pattern_len, Self { MovMUReg2_3 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:767:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_4Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_4Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 4i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_4 {
    Var0(MovMUReg2_4Var0),
    Var1(MovMUReg2_4Var1),
}
impl TableMovMUReg2_4 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_4Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_4Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:769:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_5Var0 {
    MovMUReg2_4: TableMovMUReg2_4,
}
impl MovMUReg2_5Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_4.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_4 = if let Some((len, table)) = TableMovMUReg2_4::parse(
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
        Some((pattern_len, Self { MovMUReg2_4 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:770:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_5Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_5Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 5i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_5 {
    Var0(MovMUReg2_5Var0),
    Var1(MovMUReg2_5Var1),
}
impl TableMovMUReg2_5 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_5Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_5Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:772:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_6Var0 {
    MovMUReg2_5: TableMovMUReg2_5,
}
impl MovMUReg2_6Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_5.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_5 = if let Some((len, table)) = TableMovMUReg2_5::parse(
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
        Some((pattern_len, Self { MovMUReg2_5 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:773:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_6Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_6Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 6i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_6 {
    Var0(MovMUReg2_6Var0),
    Var1(MovMUReg2_6Var1),
}
impl TableMovMUReg2_6 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_6Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_6Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:775:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_7Var0 {
    MovMUReg2_6: TableMovMUReg2_6,
}
impl MovMUReg2_7Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_6.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_6 = if let Some((len, table)) = TableMovMUReg2_6::parse(
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
        Some((pattern_len, Self { MovMUReg2_6 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:776:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_7Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_7Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 7i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_7 {
    Var0(MovMUReg2_7Var0),
    Var1(MovMUReg2_7Var1),
}
impl TableMovMUReg2_7 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_7Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_7Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:778:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_8Var0 {
    MovMUReg2_7: TableMovMUReg2_7,
}
impl MovMUReg2_8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_7.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_7 = if let Some((len, table)) = TableMovMUReg2_7::parse(
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
        Some((pattern_len, Self { MovMUReg2_7 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:779:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_8Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_8Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 8i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_8 {
    Var0(MovMUReg2_8Var0),
    Var1(MovMUReg2_8Var1),
}
impl TableMovMUReg2_8 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_8Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_8Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:781:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_9Var0 {
    MovMUReg2_8: TableMovMUReg2_8,
}
impl MovMUReg2_9Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_8 = if let Some((len, table)) = TableMovMUReg2_8::parse(
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
        Some((pattern_len, Self { MovMUReg2_8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:782:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_9Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_9Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 9i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_9 {
    Var0(MovMUReg2_9Var0),
    Var1(MovMUReg2_9Var1),
}
impl TableMovMUReg2_9 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_9Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_9Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:784:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_10Var0 {
    MovMUReg2_9: TableMovMUReg2_9,
}
impl MovMUReg2_10Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_9.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_9 = if let Some((len, table)) = TableMovMUReg2_9::parse(
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
        Some((pattern_len, Self { MovMUReg2_9 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:785:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_10Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_10Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 10i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_10 {
    Var0(MovMUReg2_10Var0),
    Var1(MovMUReg2_10Var1),
}
impl TableMovMUReg2_10 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_10Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_10Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:787:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_11Var0 {
    MovMUReg2_10: TableMovMUReg2_10,
}
impl MovMUReg2_11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_10.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_10 = if let Some((len, table)) = TableMovMUReg2_10::parse(
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
        Some((pattern_len, Self { MovMUReg2_10 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:788:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_11Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_11Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 11i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_11 {
    Var0(MovMUReg2_11Var0),
    Var1(MovMUReg2_11Var1),
}
impl TableMovMUReg2_11 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_11Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_11Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:790:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_12Var0 {
    MovMUReg2_11: TableMovMUReg2_11,
}
impl MovMUReg2_12Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_11 = if let Some((len, table)) = TableMovMUReg2_11::parse(
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
        Some((pattern_len, Self { MovMUReg2_11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:791:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_12Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_12Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 12i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_12 {
    Var0(MovMUReg2_12Var0),
    Var1(MovMUReg2_12Var1),
}
impl TableMovMUReg2_12 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_12Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_12Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:793:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_13Var0 {
    MovMUReg2_12: TableMovMUReg2_12,
}
impl MovMUReg2_13Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_12.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_12 = if let Some((len, table)) = TableMovMUReg2_12::parse(
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
        Some((pattern_len, Self { MovMUReg2_12 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:794:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_13Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_13Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 13i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_13 {
    Var0(MovMUReg2_13Var0),
    Var1(MovMUReg2_13Var1),
}
impl TableMovMUReg2_13 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_13Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_13Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:796:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_14Var0 {
    MovMUReg2_13: TableMovMUReg2_13,
}
impl MovMUReg2_14Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_13.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_13 = if let Some((len, table)) = TableMovMUReg2_13::parse(
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
        Some((pattern_len, Self { MovMUReg2_13 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:797:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_14Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_14Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 14i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_14 {
    Var0(MovMUReg2_14Var0),
    Var1(MovMUReg2_14Var1),
}
impl TableMovMUReg2_14 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_14Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_14Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:799:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_15Var0 {
    MovMUReg2_14: TableMovMUReg2_14,
}
impl MovMUReg2_15Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_14.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_14 = if let Some((len, table)) = TableMovMUReg2_14::parse(
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
        Some((pattern_len, Self { MovMUReg2_14 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:800:1"]
#[derive(Clone, Debug)]
struct MovMUReg2_15Var1 {
    rm_imm_08_11: TokenField_rm_imm_08_11,
}
impl MovMUReg2_15Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.rm_imm_08_11.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldrm_08_11().disassembly() != 15i64 {
            return None;
        }
        let rm_imm_08_11 = token_parser.TokenFieldrm_imm_08_11();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_15 {
    Var0(MovMUReg2_15Var0),
    Var1(MovMUReg2_15Var1),
}
impl TableMovMUReg2_15 {
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
        if let Some((inst_len, parsed)) = MovMUReg2_15Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = MovMUReg2_15Var1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc:802:1"]
#[derive(Clone, Debug)]
struct MovMUReg2Var0 {
    MovMUReg2_15: TableMovMUReg2_15,
}
impl MovMUReg2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.MovMUReg2_15.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let MovMUReg2_15 = if let Some((len, table)) = TableMovMUReg2_15::parse(
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
        Some((pattern_len, Self { MovMUReg2_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2 {
    Var0(MovMUReg2Var0),
}
impl TableMovMUReg2 {
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
            MovMUReg2Var0::parse(tokens_param, &mut context_current, inst_start)
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
