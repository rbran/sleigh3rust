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
        0 => Register::R0_BANK,
        1 => Register::R1_BANK,
        2 => Register::R2_BANK,
        3 => Register::R3_BANK,
        4 => Register::R4_BANK,
        5 => Register::R5_BANK,
        6 => Register::R6_BANK,
        7 => Register::R7_BANK,
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
        0 => Register::xd0,
        1 => Register::xd2,
        2 => Register::xd4,
        3 => Register::xd6,
        4 => Register::xd8,
        5 => Register::xd10,
        6 => Register::xd12,
        7 => Register::xd14,
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
        1 => Register::xd0,
        2 => Register::dr2,
        3 => Register::xd2,
        4 => Register::dr4,
        5 => Register::xd4,
        6 => Register::dr6,
        7 => Register::xd6,
        8 => Register::dr8,
        9 => Register::xd8,
        10 => Register::dr10,
        11 => Register::xd10,
        12 => Register::dr12,
        13 => Register::xd12,
        14 => Register::dr14,
        15 => Register::xd14,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_6_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_6_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_6_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::fv0,
        1 => Register::fv4,
        2 => Register::fv8,
        3 => Register::fv12,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_7_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_7_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_7_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::fr0,
        1 => Register::fr2,
        2 => Register::fr4,
        3 => Register::fr6,
        4 => Register::fr8,
        5 => Register::fr10,
        6 => Register::fr12,
        7 => Register::fr14,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_8_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_8_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_8_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::fr1,
        1 => Register::fr3,
        2 => Register::fr5,
        3 => Register::fr7,
        4 => Register::fr9,
        5 => Register::fr11,
        6 => Register::fr13,
        7 => Register::fr15,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_9_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_9_value(num.try_into().unwrap());
    DisplayElement::Register(value)
}
fn meaning_9_value<T>(num: T) -> Register
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
#[derive(Clone, Copy, Debug)]
struct TokenField_OP_0(u8);
impl TokenField_OP_0 {
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
struct TokenField_OP_1(u8);
impl TokenField_OP_1 {
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
struct TokenField_OP_2(u8);
impl TokenField_OP_2 {
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
struct TokenField_OP_3(u16);
impl TokenField_OP_3 {
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
struct TokenField_OP_4(u8);
impl TokenField_OP_4 {
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
struct TokenField_OP_5(u16);
impl TokenField_OP_5 {
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
struct TokenField_OP_6(u8);
impl TokenField_OP_6 {
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
struct TokenField_OP_7(u8);
impl TokenField_OP_7 {
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
struct TokenField_OP_8(u16);
impl TokenField_OP_8 {
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
struct TokenField_OP_9(u8);
impl TokenField_OP_9 {
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
struct TokenField_OP_10(u8);
impl TokenField_OP_10 {
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
struct TokenField_BANK(u8);
impl TokenField_BANK {
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
struct TokenField_M_0(u8);
impl TokenField_M_0 {
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
struct TokenField_M_1(u8);
impl TokenField_M_1 {
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
struct TokenField_M_2(u8);
impl TokenField_M_2 {
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
struct TokenField_FRM_0(u8);
impl TokenField_FRM_0 {
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
struct TokenField_DRM_1(u8);
impl TokenField_DRM_1 {
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
struct TokenField_XDM_1(u8);
impl TokenField_XDM_1 {
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
struct TokenField_XDRM(u8);
impl TokenField_XDRM {
    fn execution(&self) -> Register {
        meaning_5_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_5_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_FVM_2(u8);
impl TokenField_FVM_2 {
    fn execution(&self) -> Register {
        meaning_6_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_6_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_N_0(u8);
impl TokenField_N_0 {
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
struct TokenField_N_1(u8);
impl TokenField_N_1 {
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
struct TokenField_N_2(u8);
impl TokenField_N_2 {
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
struct TokenField_FRN_0(u8);
impl TokenField_FRN_0 {
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
struct TokenField_FRN_1(u8);
impl TokenField_FRN_1 {
    fn execution(&self) -> Register {
        meaning_7_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_7_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_FRN_2(u8);
impl TokenField_FRN_2 {
    fn execution(&self) -> Register {
        meaning_8_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_8_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_DRN_0(u8);
impl TokenField_DRN_0 {
    fn execution(&self) -> Register {
        meaning_9_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_9_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_DRN_1(u8);
impl TokenField_DRN_1 {
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
struct TokenField_XDN_1(u8);
impl TokenField_XDN_1 {
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
struct TokenField_XDRN(u8);
impl TokenField_XDRN {
    fn execution(&self) -> Register {
        meaning_5_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_5_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_FVN_2(u8);
impl TokenField_FVN_2 {
    fn execution(&self) -> Register {
        meaning_6_value(self.0)
    }
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_6_display(self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_I_0(i8);
impl TokenField_I_0 {
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
struct TokenField_I_1(i16);
impl TokenField_I_1 {
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
struct TokenField_I_2(i8);
impl TokenField_I_2 {
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
struct TokenField_U_0(u8);
impl TokenField_U_0 {
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
struct TokenField_U_1(u16);
impl TokenField_U_1 {
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
struct TokenField_U_2(u8);
impl TokenField_U_2 {
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
    fn TokenFieldOP_0(&self) -> TokenField_OP_0 {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_OP_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_1(&self) -> TokenField_OP_1 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_OP_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_2(&self) -> TokenField_OP_2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_OP_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_3(&self) -> TokenField_OP_3 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_OP_3(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_4(&self) -> TokenField_OP_4 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_OP_4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_5(&self) -> TokenField_OP_5 {
        let inner_value = self.read_u16::<true>(0, 0, 9).unwrap();
        TokenField_OP_5(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_6(&self) -> TokenField_OP_6 {
        let inner_value = self.read_u8::<true>(0, 0, 1).unwrap();
        TokenField_OP_6(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_7(&self) -> TokenField_OP_7 {
        let inner_value = self.read_u8::<true>(1, 0, 5).unwrap();
        TokenField_OP_7(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_8(&self) -> TokenField_OP_8 {
        let inner_value = self.read_u16::<true>(0, 0, 10).unwrap();
        TokenField_OP_8(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_9(&self) -> TokenField_OP_9 {
        let inner_value = self.read_u8::<true>(1, 7, 1).unwrap();
        TokenField_OP_9(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldOP_10(&self) -> TokenField_OP_10 {
        let inner_value = self.read_u8::<true>(1, 4, 1).unwrap();
        TokenField_OP_10(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldBANK(&self) -> TokenField_BANK {
        let inner_value = self.read_u8::<true>(1, 4, 3).unwrap();
        TokenField_BANK(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldM_0(&self) -> TokenField_M_0 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_M_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldM_1(&self) -> TokenField_M_1 {
        let inner_value = self.read_u8::<true>(1, 5, 3).unwrap();
        TokenField_M_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldM_2(&self) -> TokenField_M_2 {
        let inner_value = self.read_u8::<true>(0, 0, 2).unwrap();
        TokenField_M_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldFRM_0(&self) -> TokenField_FRM_0 {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_FRM_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldDRM_1(&self) -> TokenField_DRM_1 {
        let inner_value = self.read_u8::<true>(1, 5, 3).unwrap();
        TokenField_DRM_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldXDM_1(&self) -> TokenField_XDM_1 {
        let inner_value = self.read_u8::<true>(1, 5, 3).unwrap();
        TokenField_XDM_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldXDRM(&self) -> TokenField_XDRM {
        let inner_value = self.read_u8::<true>(1, 4, 4).unwrap();
        TokenField_XDRM(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldFVM_2(&self) -> TokenField_FVM_2 {
        let inner_value = self.read_u8::<true>(0, 0, 2).unwrap();
        TokenField_FVM_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldN_0(&self) -> TokenField_N_0 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_N_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldN_1(&self) -> TokenField_N_1 {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_N_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldN_2(&self) -> TokenField_N_2 {
        let inner_value = self.read_u8::<true>(0, 2, 2).unwrap();
        TokenField_N_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldFRN_0(&self) -> TokenField_FRN_0 {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_FRN_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldFRN_1(&self) -> TokenField_FRN_1 {
        let inner_value = self.read_u8::<true>(0, 0, 3).unwrap();
        TokenField_FRN_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldFRN_2(&self) -> TokenField_FRN_2 {
        let inner_value = self.read_u8::<true>(0, 0, 3).unwrap();
        TokenField_FRN_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldDRN_0(&self) -> TokenField_DRN_0 {
        let inner_value = self.read_u8::<true>(0, 0, 3).unwrap();
        TokenField_DRN_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldDRN_1(&self) -> TokenField_DRN_1 {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_DRN_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldXDN_1(&self) -> TokenField_XDN_1 {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_XDN_1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldXDRN(&self) -> TokenField_XDRN {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_XDRN(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldFVN_2(&self) -> TokenField_FVN_2 {
        let inner_value = self.read_u8::<true>(0, 2, 2).unwrap();
        TokenField_FVN_2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldI_0(&self) -> TokenField_I_0 {
        let inner_value = self.read_i8::<true>(1, 0, 8).unwrap();
        TokenField_I_0(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldI_1(&self) -> TokenField_I_1 {
        let inner_value = self.read_i16::<true>(0, 0, 12).unwrap();
        TokenField_I_1(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldI_2(&self) -> TokenField_I_2 {
        let inner_value = self.read_i8::<true>(1, 0, 4).unwrap();
        TokenField_I_2(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldU_0(&self) -> TokenField_U_0 {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_U_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldU_1(&self) -> TokenField_U_1 {
        let inner_value = self.read_u16::<true>(0, 0, 12).unwrap();
        TokenField_U_1(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldU_2(&self) -> TokenField_U_2 {
        let inner_value = self.read_u8::<true>(1, 0, 4).unwrap();
        TokenField_U_2(u8::try_from(inner_value).unwrap())
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
    R0_BANK,
    R1_BANK,
    R2_BANK,
    R3_BANK,
    R4_BANK,
    R5_BANK,
    R6_BANK,
    R7_BANK,
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
    xf0,
    xf1,
    xf2,
    xf3,
    xf4,
    xf5,
    xf6,
    xf7,
    xf8,
    xf9,
    xf10,
    xf11,
    xf12,
    xf13,
    xf14,
    xf15,
    dr0,
    dr2,
    dr4,
    dr6,
    dr8,
    dr10,
    dr12,
    dr14,
    xd0,
    xd2,
    xd4,
    xd6,
    xd8,
    xd10,
    xd12,
    xd14,
    fv0,
    fv4,
    fv8,
    fv12,
    GBR,
    SR,
    SSR,
    SPC,
    VBR,
    SGR,
    DBR,
    MD,
    RB,
    BL,
    FD,
    M,
    Q,
    IMASK,
    S,
    T,
    MACH,
    MACL,
    PR,
    PC,
    FPSCR,
    FPUL,
    FPSCR_RM,
    FPSCR_FLAG,
    FPSCR_ENABLE,
    FPSCR_CAUSE,
    FPSCR_DN,
    FPSCR_PR,
    FPSCR_SZ,
    FPSCR_FR,
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
            Self::R0_BANK => write!(f, "R0_BANK"),
            Self::R1_BANK => write!(f, "R1_BANK"),
            Self::R2_BANK => write!(f, "R2_BANK"),
            Self::R3_BANK => write!(f, "R3_BANK"),
            Self::R4_BANK => write!(f, "R4_BANK"),
            Self::R5_BANK => write!(f, "R5_BANK"),
            Self::R6_BANK => write!(f, "R6_BANK"),
            Self::R7_BANK => write!(f, "R7_BANK"),
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
            Self::xf0 => write!(f, "xf0"),
            Self::xf1 => write!(f, "xf1"),
            Self::xf2 => write!(f, "xf2"),
            Self::xf3 => write!(f, "xf3"),
            Self::xf4 => write!(f, "xf4"),
            Self::xf5 => write!(f, "xf5"),
            Self::xf6 => write!(f, "xf6"),
            Self::xf7 => write!(f, "xf7"),
            Self::xf8 => write!(f, "xf8"),
            Self::xf9 => write!(f, "xf9"),
            Self::xf10 => write!(f, "xf10"),
            Self::xf11 => write!(f, "xf11"),
            Self::xf12 => write!(f, "xf12"),
            Self::xf13 => write!(f, "xf13"),
            Self::xf14 => write!(f, "xf14"),
            Self::xf15 => write!(f, "xf15"),
            Self::dr0 => write!(f, "dr0"),
            Self::dr2 => write!(f, "dr2"),
            Self::dr4 => write!(f, "dr4"),
            Self::dr6 => write!(f, "dr6"),
            Self::dr8 => write!(f, "dr8"),
            Self::dr10 => write!(f, "dr10"),
            Self::dr12 => write!(f, "dr12"),
            Self::dr14 => write!(f, "dr14"),
            Self::xd0 => write!(f, "xd0"),
            Self::xd2 => write!(f, "xd2"),
            Self::xd4 => write!(f, "xd4"),
            Self::xd6 => write!(f, "xd6"),
            Self::xd8 => write!(f, "xd8"),
            Self::xd10 => write!(f, "xd10"),
            Self::xd12 => write!(f, "xd12"),
            Self::xd14 => write!(f, "xd14"),
            Self::fv0 => write!(f, "fv0"),
            Self::fv4 => write!(f, "fv4"),
            Self::fv8 => write!(f, "fv8"),
            Self::fv12 => write!(f, "fv12"),
            Self::GBR => write!(f, "GBR"),
            Self::SR => write!(f, "SR"),
            Self::SSR => write!(f, "SSR"),
            Self::SPC => write!(f, "SPC"),
            Self::VBR => write!(f, "VBR"),
            Self::SGR => write!(f, "SGR"),
            Self::DBR => write!(f, "DBR"),
            Self::MD => write!(f, "MD"),
            Self::RB => write!(f, "RB"),
            Self::BL => write!(f, "BL"),
            Self::FD => write!(f, "FD"),
            Self::M => write!(f, "M"),
            Self::Q => write!(f, "Q"),
            Self::IMASK => write!(f, "IMASK"),
            Self::S => write!(f, "S"),
            Self::T => write!(f, "T"),
            Self::MACH => write!(f, "MACH"),
            Self::MACL => write!(f, "MACL"),
            Self::PR => write!(f, "PR"),
            Self::PC => write!(f, "PC"),
            Self::FPSCR => write!(f, "FPSCR"),
            Self::FPUL => write!(f, "FPUL"),
            Self::FPSCR_RM => write!(f, "FPSCR_RM"),
            Self::FPSCR_FLAG => write!(f, "FPSCR_FLAG"),
            Self::FPSCR_ENABLE => write!(f, "FPSCR_ENABLE"),
            Self::FPSCR_CAUSE => write!(f, "FPSCR_CAUSE"),
            Self::FPSCR_DN => write!(f, "FPSCR_DN"),
            Self::FPSCR_PR => write!(f, "FPSCR_PR"),
            Self::FPSCR_SZ => write!(f, "FPSCR_SZ"),
            Self::FPSCR_FR => write!(f, "FPSCR_FR"),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:796:1, end:796:2))"]
#[derive(Clone, Debug)]
struct clrmac_instructionVar0 {}
impl clrmac_instructionVar0 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 40i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:807:1, end:807:2))"]
#[derive(Clone, Debug)]
struct clrs_instructionVar1 {}
impl clrs_instructionVar1 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("clrs")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_3().disassembly() != 72i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:817:1, end:817:2))"]
#[derive(Clone, Debug)]
struct clrt_instructionVar2 {}
impl clrt_instructionVar2 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 8i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:933:1, end:933:2))"]
#[derive(Clone, Debug)]
struct div0u_instructionVar3 {}
impl div0u_instructionVar3 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 25i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1459:1, end:1459:2))"]
#[derive(Clone, Debug)]
struct frchg_instructionVar4 {}
impl frchg_instructionVar4 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("frchg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_3().disassembly() != 64509i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1473:1, end:1473:2))"]
#[derive(Clone, Debug)]
struct fschg_instructionVar5 {}
impl fschg_instructionVar5 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 62461i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1485:1, end:1485:2))"]
#[derive(Clone, Debug)]
struct fpchg_instructionVar6 {}
impl fpchg_instructionVar6 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("fpchg")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_3().disassembly() != 63485i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1909:1, end:1909:2))"]
#[derive(Clone, Debug)]
struct ldtlb_instructionVar7 {}
impl ldtlb_instructionVar7 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldtlb")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_3().disassembly() != 56i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2346:1, end:2346:2))"]
#[derive(Clone, Debug)]
struct movua_l_instructionVar8 {
    N_0t_at: TableN_0t_at,
}
impl movua_l_instructionVar8 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movua.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at.display_extend(
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
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldN_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 233i128 {
            return None;
        }
        let N_0t_at = if let Some((len, table)) = TableN_0t_at::parse(
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
        Some((pattern_len, Self { N_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2407:1, end:2407:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar9 {}
impl nop_instructionVar9 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 9i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2531:1, end:2531:2))"]
#[derive(Clone, Debug)]
struct rte_instructionVar10 {}
impl rte_instructionVar10 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 43i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2545:1, end:2545:2))"]
#[derive(Clone, Debug)]
struct rts_instructionVar11 {}
impl rts_instructionVar11 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 11i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2557:1, end:2557:2))"]
#[derive(Clone, Debug)]
struct sets_instructionVar12 {}
impl sets_instructionVar12 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("sets")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_3().disassembly() != 88i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2567:1, end:2567:2))"]
#[derive(Clone, Debug)]
struct sett_instructionVar13 {}
impl sett_instructionVar13 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 24i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2709:1, end:2709:2))"]
#[derive(Clone, Debug)]
struct sleep_instructionVar14 {}
impl sleep_instructionVar14 {
    fn display_extend<T>(
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
        if token_parser.TokenFieldOP_3().disassembly() != 27i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3040:1, end:3040:2))"]
#[derive(Clone, Debug)]
struct synco_instructionVar15 {}
impl synco_instructionVar15 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("synco")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_3().disassembly() != 171i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1586:1, end:1586:2))"]
#[derive(Clone, Debug)]
struct ftrv_instructionVar16 {
    FVN_2t: TableFVN_2t,
    XMTRX_t: TableXMTRX_t,
}
impl ftrv_instructionVar16 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ftrv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XMTRX_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FVN_2t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_8().disassembly() != 509i128 {
            return None;
        }
        let FVN_2t = if let Some((len, table)) = TableFVN_2t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let XMTRX_t = if let Some((len, table)) = TableXMTRX_t::parse(
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
        Some((pattern_len, Self { FVN_2t, XMTRX_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1192:1, end:1192:2))"]
#[derive(Clone, Debug)]
struct fcnvds_instructionVar17 {
    DRN_1t: TableDRN_1t,
}
impl fcnvds_instructionVar17 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fcnvds"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.DRN_1t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_5().disassembly() != 189i128 {
            return None;
        }
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
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
        Some((pattern_len, Self { DRN_1t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1205:1, end:1205:2))"]
#[derive(Clone, Debug)]
struct fcnvsd_instructionVar18 {
    DRN_1t: TableDRN_1t,
}
impl fcnvsd_instructionVar18 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fcnvsd"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DRN_1t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_5().disassembly() != 173i128 {
            return None;
        }
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
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
        Some((pattern_len, Self { DRN_1t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:739:1, end:739:2))"]
#[derive(Clone, Debug)]
struct braf_instructionVar19 {
    N_0: TokenField_N_0,
}
impl braf_instructionVar19 {
    fn display_extend<T>(
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
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 35i128 {
            return None;
        }
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:762:1, end:762:2))"]
#[derive(Clone, Debug)]
struct bsrf_instructionVar20 {
    N_0: TokenField_N_0,
}
impl bsrf_instructionVar20 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 3i128 {
            return None;
        }
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:876:1, end:876:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar21 {
    N_0t: TableN_0t,
}
impl cmp_instructionVar21 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/pl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 21i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:885:1, end:885:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar22 {
    N_0t: TableN_0t,
}
impl cmp_instructionVar22 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/pz"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 17i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1080:1, end:1080:2))"]
#[derive(Clone, Debug)]
struct dt_instructionVar23 {
    N_0t: TableN_0t,
}
impl dt_instructionVar23 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("dt"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 16i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1130:1, end:1130:2))"]
#[derive(Clone, Debug)]
struct fabs_instructionVar24 {
    FRN_0: TokenField_FRN_0,
}
impl fabs_instructionVar24 {
    fn display_extend<T>(
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
            self.FRN_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 93i128 {
            return None;
        }
        let FRN_0 = token_parser.TokenFieldFRN_0();
        let DRN_1 = token_parser.TokenFieldDRN_1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRN_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1233:1, end:1233:2))"]
#[derive(Clone, Debug)]
struct fipr_instructionVar25 {
    FVN_2t: TableFVN_2t,
    FVM_2t: TableFVM_2t,
}
impl fipr_instructionVar25 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fipr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FVM_2t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FVN_2t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 237i128 {
            return None;
        }
        let FVN_2t = if let Some((len, table)) = TableFVN_2t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FVM_2t = if let Some((len, table)) = TableFVM_2t::parse(
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
        Some((pattern_len, Self { FVN_2t, FVM_2t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1255:1, end:1255:2))"]
#[derive(Clone, Debug)]
struct fldi0_instructionVar26 {
    DRN_1t: TableDRN_1t,
    FRN_0t: TableFRN_0t,
}
impl fldi0_instructionVar26 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fldi0"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 141i128 {
            return None;
        }
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
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
        Some((pattern_len, Self { DRN_1t, FRN_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1268:1, end:1268:2))"]
#[derive(Clone, Debug)]
struct fldi1_instructionVar27 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
}
impl fldi1_instructionVar27 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fldi1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 157i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
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
        Some((pattern_len, Self { FRN_0t, DRN_1t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1278:1, end:1278:2))"]
#[derive(Clone, Debug)]
struct flds_instructionVar28 {
    FRN_0t: TableFRN_0t,
}
impl flds_instructionVar28 {
    fn display_extend<T>(
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
            DisplayElement::Literal("flds"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 29i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
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
        Some((pattern_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1287:1, end:1287:2))"]
#[derive(Clone, Debug)]
struct float_instructionVar29 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
}
impl float_instructionVar29 {
    fn display_extend<T>(
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
            DisplayElement::Literal("float"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 45i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let OP_6 = token_parser.TokenFieldOP_6();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRN_0t, DRN_1t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1444:1, end:1444:2))"]
#[derive(Clone, Debug)]
struct fneg_instructionVar30 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
}
impl fneg_instructionVar30 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fneg"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 77i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let OP_6 = token_parser.TokenFieldOP_6();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRN_0t, DRN_1t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1494:1, end:1494:2))"]
#[derive(Clone, Debug)]
struct fsqrt_instructionVar31 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
}
impl fsqrt_instructionVar31 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fsqrt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 109i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
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
        Some((pattern_len, Self { FRN_0t, DRN_1t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1509:1, end:1509:2))"]
#[derive(Clone, Debug)]
struct fsrra_instructionVar32 {
    FRN_0: TokenField_FRN_0,
}
impl fsrra_instructionVar32 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fsrra"),
            DisplayElement::Literal(" "),
            self.FRN_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 125i128 {
            return None;
        }
        let FRN_0 = token_parser.TokenFieldFRN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRN_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1522:1, end:1522:2))"]
#[derive(Clone, Debug)]
struct fsca_instructionVar33 {
    DRN_0: TokenField_DRN_0,
}
impl fsca_instructionVar33 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fsca"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
            self.DRN_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 253i128 {
            return None;
        }
        let DRN_0 = token_parser.TokenFieldDRN_0();
        let FRN_1 = token_parser.TokenFieldFRN_1();
        let FRN_2 = token_parser.TokenFieldFRN_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DRN_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1546:1, end:1546:2))"]
#[derive(Clone, Debug)]
struct fsts_instructionVar34 {
    FRN_0t: TableFRN_0t,
}
impl fsts_instructionVar34 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fsts"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 13i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
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
        Some((pattern_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1571:1, end:1571:2))"]
#[derive(Clone, Debug)]
struct ftrc_instructionVar35 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
}
impl ftrc_instructionVar35 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ftrc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 61i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
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
        Some((pattern_len, Self { FRN_0t, DRN_1t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1623:1, end:1623:2))"]
#[derive(Clone, Debug)]
struct icbi_instructionVar36 {
    N_0t_at1: TableN_0t_at1,
}
impl icbi_instructionVar36 {
    fn display_extend<T>(
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
            DisplayElement::Literal("icbi"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 227i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1632:1, end:1632:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar37 {
    N_0tjmp: TableN_0tjmp,
}
impl jmp_instructionVar37 {
    fn display_extend<T>(
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
        self.N_0tjmp.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 43i128 {
            return None;
        }
        let N_0tjmp = if let Some((len, table)) = TableN_0tjmp::parse(
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
        Some((pattern_len, Self { N_0tjmp }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1645:1, end:1645:2))"]
#[derive(Clone, Debug)]
struct jsr_instructionVar38 {
    N_0tjmp: TableN_0tjmp,
}
impl jsr_instructionVar38 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("jsr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0tjmp.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 11i128 {
            return None;
        }
        let N_0tjmp = if let Some((len, table)) = TableN_0tjmp::parse(
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
        Some((pattern_len, Self { N_0tjmp }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1659:1, end:1659:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar39 {
    N_0t_sr: TableN_0t_sr,
}
impl ldc_instructionVar39 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_sr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 14i128 {
            return None;
        }
        let N_0t_sr = if let Some((len, table)) = TableN_0t_sr::parse(
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
        Some((pattern_len, Self { N_0t_sr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1670:1, end:1670:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar40 {
    N_0t_gbr: TableN_0t_gbr,
}
impl ldc_instructionVar40 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_gbr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 30i128 {
            return None;
        }
        let N_0t_gbr = if let Some((len, table)) = TableN_0t_gbr::parse(
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
        Some((pattern_len, Self { N_0t_gbr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1680:1, end:1680:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar41 {
    N_0t_vbr: TableN_0t_vbr,
}
impl ldc_instructionVar41 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_vbr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 46i128 {
            return None;
        }
        let N_0t_vbr = if let Some((len, table)) = TableN_0t_vbr::parse(
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
        Some((pattern_len, Self { N_0t_vbr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1689:1, end:1689:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar42 {
    N_0t_ssr: TableN_0t_ssr,
}
impl ldc_instructionVar42 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_ssr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 62i128 {
            return None;
        }
        let N_0t_ssr = if let Some((len, table)) = TableN_0t_ssr::parse(
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
        Some((pattern_len, Self { N_0t_ssr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1698:1, end:1698:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar43 {
    N_0t_spc: TableN_0t_spc,
}
impl ldc_instructionVar43 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_spc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 78i128 {
            return None;
        }
        let N_0t_spc = if let Some((len, table)) = TableN_0t_spc::parse(
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
        Some((pattern_len, Self { N_0t_spc }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1707:1, end:1707:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar44 {
    N_0t_dbr: TableN_0t_dbr,
}
impl ldc_instructionVar44 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_dbr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 250i128 {
            return None;
        }
        let N_0t_dbr = if let Some((len, table)) = TableN_0t_dbr::parse(
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
        Some((pattern_len, Self { N_0t_dbr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1725:1, end:1725:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar45 {
    N_0t_sr1: TableN_0t_sr1,
}
impl ldc_l_instructionVar45 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_sr1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 7i128 {
            return None;
        }
        let N_0t_sr1 = if let Some((len, table)) = TableN_0t_sr1::parse(
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
        Some((pattern_len, Self { N_0t_sr1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1737:1, end:1737:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar46 {
    N_0t_gbr1: TableN_0t_gbr1,
}
impl ldc_l_instructionVar46 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_gbr1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 23i128 {
            return None;
        }
        let N_0t_gbr1 = if let Some((len, table)) = TableN_0t_gbr1::parse(
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
        Some((pattern_len, Self { N_0t_gbr1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1748:1, end:1748:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar47 {
    N_0t_vbr1: TableN_0t_vbr1,
}
impl ldc_l_instructionVar47 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_vbr1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 39i128 {
            return None;
        }
        let N_0t_vbr1 = if let Some((len, table)) = TableN_0t_vbr1::parse(
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
        Some((pattern_len, Self { N_0t_vbr1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1758:1, end:1758:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar48 {
    N_0t_ssr1: TableN_0t_ssr1,
}
impl ldc_l_instructionVar48 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_ssr1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 55i128 {
            return None;
        }
        let N_0t_ssr1 = if let Some((len, table)) = TableN_0t_ssr1::parse(
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
        Some((pattern_len, Self { N_0t_ssr1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1768:1, end:1768:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar49 {
    N_0t_spc1: TableN_0t_spc1,
}
impl ldc_l_instructionVar49 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_spc1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 71i128 {
            return None;
        }
        let N_0t_spc1 = if let Some((len, table)) = TableN_0t_spc1::parse(
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
        Some((pattern_len, Self { N_0t_spc1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1778:1, end:1778:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar50 {
    N_0t_dbr1: TableN_0t_dbr1,
}
impl ldc_l_instructionVar50 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_dbr1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 246i128 {
            return None;
        }
        let N_0t_dbr1 = if let Some((len, table)) = TableN_0t_dbr1::parse(
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
        Some((pattern_len, Self { N_0t_dbr1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1799:1, end:1799:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar51 {
    N_0t_fpul: TableN_0t_fpul,
}
impl lds_instructionVar51 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_fpul.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 90i128 {
            return None;
        }
        let N_0t_fpul = if let Some((len, table)) = TableN_0t_fpul::parse(
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
        Some((pattern_len, Self { N_0t_fpul }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1809:1, end:1809:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar52 {
    N_0t_fpul1: TableN_0t_fpul1,
}
impl lds_l_instructionVar52 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_fpul1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 86i128 {
            return None;
        }
        let N_0t_fpul1 = if let Some((len, table)) = TableN_0t_fpul1::parse(
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
        Some((pattern_len, Self { N_0t_fpul1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1821:1, end:1821:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar53 {
    N_0t_fpscr: TableN_0t_fpscr,
}
impl lds_instructionVar53 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_fpscr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 106i128 {
            return None;
        }
        let N_0t_fpscr = if let Some((len, table)) = TableN_0t_fpscr::parse(
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
        Some((pattern_len, Self { N_0t_fpscr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1833:1, end:1833:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar54 {
    N_0t_fpscr1: TableN_0t_fpscr1,
}
impl lds_l_instructionVar54 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_fpscr1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 102i128 {
            return None;
        }
        let N_0t_fpscr1 = if let Some((len, table)) = TableN_0t_fpscr1::parse(
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
        Some((pattern_len, Self { N_0t_fpscr1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1845:1, end:1845:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar55 {
    N_0t_mach: TableN_0t_mach,
}
impl lds_instructionVar55 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_mach.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 10i128 {
            return None;
        }
        let N_0t_mach = if let Some((len, table)) = TableN_0t_mach::parse(
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
        Some((pattern_len, Self { N_0t_mach }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1855:1, end:1855:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar56 {
    N_0t_macl: TableN_0t_macl,
}
impl lds_instructionVar56 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_macl.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 26i128 {
            return None;
        }
        let N_0t_macl = if let Some((len, table)) = TableN_0t_macl::parse(
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
        Some((pattern_len, Self { N_0t_macl }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1865:1, end:1865:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar57 {
    N_0t_pr: TableN_0t_pr,
}
impl lds_instructionVar57 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_pr.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 42i128 {
            return None;
        }
        let N_0t_pr = if let Some((len, table)) = TableN_0t_pr::parse(
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
        Some((pattern_len, Self { N_0t_pr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1875:1, end:1875:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar58 {
    N_0t_mach1: TableN_0t_mach1,
}
impl lds_l_instructionVar58 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_mach1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 6i128 {
            return None;
        }
        let N_0t_mach1 = if let Some((len, table)) = TableN_0t_mach1::parse(
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
        Some((pattern_len, Self { N_0t_mach1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1886:1, end:1886:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar59 {
    N_0t_macl1: TableN_0t_macl1,
}
impl lds_l_instructionVar59 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_macl1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 22i128 {
            return None;
        }
        let N_0t_macl1 = if let Some((len, table)) = TableN_0t_macl1::parse(
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
        Some((pattern_len, Self { N_0t_macl1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1897:1, end:1897:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar60 {
    N_0t_pr1: TableN_0t_pr1,
}
impl lds_l_instructionVar60 {
    fn display_extend<T>(
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
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_pr1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 38i128 {
            return None;
        }
        let N_0t_pr1 = if let Some((len, table)) = TableN_0t_pr1::parse(
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
        Some((pattern_len, Self { N_0t_pr1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2315:1, end:2315:2))"]
#[derive(Clone, Debug)]
struct movca_l_instructionVar61 {
    N_0txx: TableN_0txx,
}
impl movca_l_instructionVar61 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movca.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0txx.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 195i128 {
            return None;
        }
        let N_0txx = if let Some((len, table)) = TableN_0txx::parse(
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
        Some((pattern_len, Self { N_0txx }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2324:1, end:2324:2))"]
#[derive(Clone, Debug)]
struct movt_instructionVar62 {
    N_0t: TableN_0t,
}
impl movt_instructionVar62 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 41i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2333:1, end:2333:2))"]
#[derive(Clone, Debug)]
struct movua_l_instructionVar63 {
    N_0t_at: TableN_0t_at,
}
impl movua_l_instructionVar63 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movua.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at.display_extend(
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
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 169i128 {
            return None;
        }
        let N_0t_at = if let Some((len, table)) = TableN_0t_at::parse(
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
        Some((pattern_len, Self { N_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2341:1, end:2341:2))"]
#[derive(Clone, Debug)]
struct movua_l_instructionVar64 {
    N_0t_at: TableN_0t_at,
}
impl movua_l_instructionVar64 {
    fn display_extend<T>(
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
            DisplayElement::Literal("movua.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at.display_extend(
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
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 233i128 {
            return None;
        }
        let N_0t_at = if let Some((len, table)) = TableN_0t_at::parse(
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
        Some((pattern_len, Self { N_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2425:1, end:2425:2))"]
#[derive(Clone, Debug)]
struct ocbi_instructionVar65 {
    N_0t_at1: TableN_0t_at1,
}
impl ocbi_instructionVar65 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ocbi"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 147i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2433:1, end:2433:2))"]
#[derive(Clone, Debug)]
struct ocbp_instructionVar66 {
    N_0t_at1: TableN_0t_at1,
}
impl ocbp_instructionVar66 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ocbp"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 163i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2442:1, end:2442:2))"]
#[derive(Clone, Debug)]
struct ocbwb_instructionVar67 {
    N_0t_at1: TableN_0t_at1,
}
impl ocbwb_instructionVar67 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ocbwb"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 179i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2479:1, end:2479:2))"]
#[derive(Clone, Debug)]
struct pref_instructionVar68 {
    N_0tjmp: TableN_0tjmp,
}
impl pref_instructionVar68 {
    fn display_extend<T>(
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
            DisplayElement::Literal("pref"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0tjmp.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 131i128 {
            return None;
        }
        let N_0tjmp = if let Some((len, table)) = TableN_0tjmp::parse(
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
        Some((pattern_len, Self { N_0tjmp }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2486:1, end:2486:2))"]
#[derive(Clone, Debug)]
struct rotcl_instructionVar69 {
    N_0t: TableN_0t,
}
impl rotcl_instructionVar69 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rotcl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 36i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2497:1, end:2497:2))"]
#[derive(Clone, Debug)]
struct rotcr_instructionVar70 {
    N_0t: TableN_0t,
}
impl rotcr_instructionVar70 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rotcr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 37i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2509:1, end:2509:2))"]
#[derive(Clone, Debug)]
struct rotl_instructionVar71 {
    N_0t: TableN_0t,
}
impl rotl_instructionVar71 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rotl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 4i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2520:1, end:2520:2))"]
#[derive(Clone, Debug)]
struct rotr_instructionVar72 {
    N_0t: TableN_0t,
}
impl rotr_instructionVar72 {
    fn display_extend<T>(
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
            DisplayElement::Literal("rotr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 5i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2591:1, end:2591:2))"]
#[derive(Clone, Debug)]
struct shal_instructionVar73 {
    N_0t: TableN_0t,
}
impl shal_instructionVar73 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shal"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 32i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2602:1, end:2602:2))"]
#[derive(Clone, Debug)]
struct shar_instructionVar74 {
    N_0t: TableN_0t,
}
impl shar_instructionVar74 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shar"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 33i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2628:1, end:2628:2))"]
#[derive(Clone, Debug)]
struct shll_instructionVar75 {
    N_0t: TableN_0t,
}
impl shll_instructionVar75 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shll"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 0i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2639:1, end:2639:2))"]
#[derive(Clone, Debug)]
struct shll2_instructionVar76 {
    N_0t: TableN_0t,
}
impl shll2_instructionVar76 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shll2"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 8i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2649:1, end:2649:2))"]
#[derive(Clone, Debug)]
struct shll8_instructionVar77 {
    N_0t: TableN_0t,
}
impl shll8_instructionVar77 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shll8"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 24i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2658:1, end:2658:2))"]
#[derive(Clone, Debug)]
struct shll16_instructionVar78 {
    N_0t: TableN_0t,
}
impl shll16_instructionVar78 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shll16"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 40i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2668:1, end:2668:2))"]
#[derive(Clone, Debug)]
struct shlr_instructionVar79 {
    N_0t: TableN_0t,
}
impl shlr_instructionVar79 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shlr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 1i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2679:1, end:2679:2))"]
#[derive(Clone, Debug)]
struct shlr2_instructionVar80 {
    N_0t: TableN_0t,
}
impl shlr2_instructionVar80 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shlr2"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 9i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2689:1, end:2689:2))"]
#[derive(Clone, Debug)]
struct shlr8_instructionVar81 {
    N_0t: TableN_0t,
}
impl shlr8_instructionVar81 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shlr8"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 25i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2699:1, end:2699:2))"]
#[derive(Clone, Debug)]
struct shlr16_instructionVar82 {
    N_0t: TableN_0t,
}
impl shlr16_instructionVar82 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shlr16"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 41i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2716:1, end:2716:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar83 {
    sr_N_0t: Tablesr_N_0t,
}
impl stc_instructionVar83 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.sr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 2i128 {
            return None;
        }
        let sr_N_0t = if let Some((len, table)) = Tablesr_N_0t::parse(
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
        Some((pattern_len, Self { sr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2727:1, end:2727:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar84 {
    gbr_N_0t: Tablegbr_N_0t,
}
impl stc_instructionVar84 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.gbr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 18i128 {
            return None;
        }
        let gbr_N_0t = if let Some((len, table)) = Tablegbr_N_0t::parse(
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
        Some((pattern_len, Self { gbr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2737:1, end:2737:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar85 {
    vbr_N_0t: Tablevbr_N_0t,
}
impl stc_instructionVar85 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.vbr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 34i128 {
            return None;
        }
        let vbr_N_0t = if let Some((len, table)) = Tablevbr_N_0t::parse(
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
        Some((pattern_len, Self { vbr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2746:1, end:2746:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar86 {
    ssr_N_0t: Tablessr_N_0t,
}
impl stc_instructionVar86 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ssr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 50i128 {
            return None;
        }
        let ssr_N_0t = if let Some((len, table)) = Tablessr_N_0t::parse(
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
        Some((pattern_len, Self { ssr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2756:1, end:2756:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar87 {
    spc_N_0t: Tablespc_N_0t,
}
impl stc_instructionVar87 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.spc_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 66i128 {
            return None;
        }
        let spc_N_0t = if let Some((len, table)) = Tablespc_N_0t::parse(
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
        Some((pattern_len, Self { spc_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2765:1, end:2765:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar88 {
    sgr_N_0t: Tablesgr_N_0t,
}
impl stc_instructionVar88 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.sgr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 58i128 {
            return None;
        }
        let sgr_N_0t = if let Some((len, table)) = Tablesgr_N_0t::parse(
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
        Some((pattern_len, Self { sgr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2774:1, end:2774:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar89 {
    dbr_N_0t: Tabledbr_N_0t,
}
impl stc_instructionVar89 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dbr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 250i128 {
            return None;
        }
        let dbr_N_0t = if let Some((len, table)) = Tabledbr_N_0t::parse(
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
        Some((pattern_len, Self { dbr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2793:1, end:2793:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar90 {
    N_0t_at_neg: TableN_0t_at_neg,
    sr_t: Tablesr_t,
}
impl stc_l_instructionVar90 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.sr_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 3i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sr_t = if let Some((len, table)) =
            Tablesr_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, sr_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2805:1, end:2805:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar91 {
    N_0t_at_neg: TableN_0t_at_neg,
    gbr_t: Tablegbr_t,
}
impl stc_l_instructionVar91 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.gbr_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 19i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let gbr_t = if let Some((len, table)) =
            Tablegbr_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, gbr_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2816:1, end:2816:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar92 {
    N_0t_at_neg: TableN_0t_at_neg,
    vbr_t: Tablevbr_t,
}
impl stc_l_instructionVar92 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.vbr_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 35i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let vbr_t = if let Some((len, table)) =
            Tablevbr_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, vbr_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2826:1, end:2826:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar93 {
    N_0t_at_neg: TableN_0t_at_neg,
    ssr_t: Tablessr_t,
}
impl stc_l_instructionVar93 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.ssr_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 51i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let ssr_t = if let Some((len, table)) =
            Tablessr_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, ssr_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2836:1, end:2836:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar94 {
    N_0t_at_neg: TableN_0t_at_neg,
    spc_t: Tablespc_t,
}
impl stc_l_instructionVar94 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.spc_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 67i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let spc_t = if let Some((len, table)) =
            Tablespc_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, spc_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2846:1, end:2846:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar95 {
    N_0t_at_neg: TableN_0t_at_neg,
    sgr_t: Tablesgr_t,
}
impl stc_l_instructionVar95 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.sgr_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 50i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let sgr_t = if let Some((len, table)) =
            Tablesgr_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, sgr_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2856:1, end:2856:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar96 {
    N_0t_at_neg: TableN_0t_at_neg,
    dbr_t: Tabledbr_t,
}
impl stc_l_instructionVar96 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.dbr_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 242i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let dbr_t = if let Some((len, table)) =
            Tabledbr_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, dbr_t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2877:1, end:2877:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar97 {
    mach_N_0t: Tablemach_N_0t,
}
impl sts_instructionVar97 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.mach_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 10i128 {
            return None;
        }
        let mach_N_0t = if let Some((len, table)) = Tablemach_N_0t::parse(
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
        Some((pattern_len, Self { mach_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2887:1, end:2887:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar98 {
    macl_N_0t: Tablemacl_N_0t,
}
impl sts_instructionVar98 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.macl_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 26i128 {
            return None;
        }
        let macl_N_0t = if let Some((len, table)) = Tablemacl_N_0t::parse(
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
        Some((pattern_len, Self { macl_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2897:1, end:2897:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar99 {
    pr_N_0t: Tablepr_N_0t,
}
impl sts_instructionVar99 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.pr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 42i128 {
            return None;
        }
        let pr_N_0t = if let Some((len, table)) = Tablepr_N_0t::parse(
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
        Some((pattern_len, Self { pr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2907:1, end:2907:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar100 {
    N_0t_at_neg: TableN_0t_at_neg,
    mach_t: Tablemach_t,
}
impl sts_l_instructionVar100 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.mach_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 2i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let mach_t = if let Some((len, table)) = Tablemach_t::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t_at_neg,
                mach_t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2918:1, end:2918:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar101 {
    N_0t_at_neg: TableN_0t_at_neg,
    macl_t: Tablemacl_t,
}
impl sts_l_instructionVar101 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.macl_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 18i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let macl_t = if let Some((len, table)) = Tablemacl_t::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t_at_neg,
                macl_t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2929:1, end:2929:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar102 {
    N_0t_at_neg: TableN_0t_at_neg,
}
impl sts_l_instructionVar102 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::PR),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 34i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
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
        Some((pattern_len, Self { N_0t_at_neg }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2940:1, end:2940:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar103 {
    fpul_N_0t: Tablefpul_N_0t,
}
impl sts_instructionVar103 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.fpul_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 90i128 {
            return None;
        }
        let fpul_N_0t = if let Some((len, table)) = Tablefpul_N_0t::parse(
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
        Some((pattern_len, Self { fpul_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2950:1, end:2950:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar104 {
    fpscr_N_0t: Tablefpscr_N_0t,
}
impl sts_instructionVar104 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.fpscr_N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 106i128 {
            return None;
        }
        let fpscr_N_0t = if let Some((len, table)) = Tablefpscr_N_0t::parse(
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
        Some((pattern_len, Self { fpscr_N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2961:1, end:2961:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar105 {
    N_0t_at_neg: TableN_0t_at_neg,
    fpul_t: Tablefpul_t,
}
impl sts_l_instructionVar105 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.fpul_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 82i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let fpul_t = if let Some((len, table)) = Tablefpul_t::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t_at_neg,
                fpul_t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2972:1, end:2972:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar106 {
    N_0t_at_neg: TableN_0t_at_neg,
    fpscr_t: Tablefpscr_t,
}
impl sts_l_instructionVar106 {
    fn display_extend<T>(
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
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.fpscr_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 98i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let fpscr_t = if let Some((len, table)) = Tablefpscr_t::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t_at_neg,
                fpscr_t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3049:1, end:3049:2))"]
#[derive(Clone, Debug)]
struct tas_b_instructionVar107 {
    N_0t_at1: TableN_0t_at1,
}
impl tas_b_instructionVar107 {
    fn display_extend<T>(
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
            DisplayElement::Literal("tas.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_4().disassembly() != 27i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1715:1, end:1715:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar108 {
    N_0t_bank: TableN_0t_bank,
    BANKt: TableBANKt,
}
impl ldc_instructionVar108 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.N_0t_bank.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BANKt.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_9().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 14i128 {
            return None;
        }
        let N_0t_bank = if let Some((len, table)) = TableN_0t_bank::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BANKt = if let Some((len, table)) =
            TableBANKt::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_bank, BANKt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1788:1, end:1788:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar109 {
    N_0t_bank1: TableN_0t_bank1,
    BANKt: TableBANKt,
}
impl ldc_l_instructionVar109 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_bank1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.BANKt.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_9().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 7i128 {
            return None;
        }
        let N_0t_bank1 = if let Some((len, table)) = TableN_0t_bank1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BANKt = if let Some((len, table)) =
            TableBANKt::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_bank1, BANKt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2783:1, end:2783:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar110 {
    N_0t_bank: TableN_0t_bank,
    BANKt: TableBANKt,
}
impl stc_instructionVar110 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.BANKt.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_bank.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_9().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 2i128 {
            return None;
        }
        let N_0t_bank = if let Some((len, table)) = TableN_0t_bank::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BANKt = if let Some((len, table)) =
            TableBANKt::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_bank, BANKt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2866:1, end:2866:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar111 {
    N_0t_at_neg: TableN_0t_at_neg,
    BANKt: TableBANKt,
}
impl stc_l_instructionVar111 {
    fn display_extend<T>(
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
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.BANKt.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_9().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 3i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let BANKt = if let Some((len, table)) =
            TableBANKt::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, BANKt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:634:1, end:634:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar112 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl add_instructionVar112 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 12i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:654:1, end:654:2))"]
#[derive(Clone, Debug)]
struct addc_instructionVar113 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl addc_instructionVar113 {
    fn display_extend<T>(
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
            DisplayElement::Literal("addc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 14i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:667:1, end:667:2))"]
#[derive(Clone, Debug)]
struct addv_instructionVar114 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl addv_instructionVar114 {
    fn display_extend<T>(
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
            DisplayElement::Literal("addv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 15i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:678:1, end:678:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar115 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl and_instructionVar115 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 9i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:687:1, end:687:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar116 {
    U_0t_r0: TableU_0t_r0,
}
impl and_instructionVar116 {
    fn display_extend<T>(
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
        self.U_0t_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 201i128 {
            return None;
        }
        let U_0t_r0 = if let Some((len, table)) = TableU_0t_r0::parse(
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
        Some((pattern_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:696:1, end:696:2))"]
#[derive(Clone, Debug)]
struct and_b_instructionVar117 {
    U_0t1: TableU_0t1,
}
impl and_b_instructionVar117 {
    fn display_extend<T>(
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
            DisplayElement::Literal("and.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 205i128 {
            return None;
        }
        let U_0t1 = if let Some((len, table)) =
            TableU_0t1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:706:1, end:706:2))"]
#[derive(Clone, Debug)]
struct bf_instructionVar118 {
    I_0tbranch: TableI_0tbranch,
}
impl bf_instructionVar118 {
    fn display_extend<T>(
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
        self.I_0tbranch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 139i128 {
            return None;
        }
        let I_0tbranch = if let Some((len, table)) = TableI_0tbranch::parse(
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
        Some((pattern_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:716:1, end:716:2))"]
#[derive(Clone, Debug)]
struct bf_instructionVar119 {
    I_0tbranch: TableI_0tbranch,
}
impl bf_instructionVar119 {
    fn display_extend<T>(
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
        self.I_0tbranch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 143i128 {
            return None;
        }
        let I_0tbranch = if let Some((len, table)) = TableI_0tbranch::parse(
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
        Some((pattern_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:775:1, end:775:2))"]
#[derive(Clone, Debug)]
struct bt_instructionVar120 {
    I_0tbranch: TableI_0tbranch,
}
impl bt_instructionVar120 {
    fn display_extend<T>(
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
        self.I_0tbranch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 137i128 {
            return None;
        }
        let I_0tbranch = if let Some((len, table)) = TableI_0tbranch::parse(
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
        Some((pattern_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:785:1, end:785:2))"]
#[derive(Clone, Debug)]
struct bt_instructionVar121 {
    I_0tbranch: TableI_0tbranch,
}
impl bt_instructionVar121 {
    fn display_extend<T>(
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
        self.I_0tbranch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 141i128 {
            return None;
        }
        let I_0tbranch = if let Some((len, table)) = TableI_0tbranch::parse(
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
        Some((pattern_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:827:1, end:827:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar122 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl cmp_instructionVar122 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 0i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:837:1, end:837:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar123 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl cmp_instructionVar123 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/ge"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 3i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:846:1, end:846:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar124 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl cmp_instructionVar124 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/gt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 7i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:856:1, end:856:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar125 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl cmp_instructionVar125 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/hi"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 6i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:866:1, end:866:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar126 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl cmp_instructionVar126 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/hs"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 2i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:895:1, end:895:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar127 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl cmp_instructionVar127 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/str"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 12i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:911:1, end:911:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar128 {
    I_0t_r0: TableI_0t_r0,
}
impl cmp_instructionVar128 {
    fn display_extend<T>(
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
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.I_0t_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 136i128 {
            return None;
        }
        let I_0t_r0 = if let Some((len, table)) = TableI_0t_r0::parse(
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
        Some((pattern_len, Self { I_0t_r0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:921:1, end:921:2))"]
#[derive(Clone, Debug)]
struct div0s_instructionVar129 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl div0s_instructionVar129 {
    fn display_extend<T>(
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
            DisplayElement::Literal("div0s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 7i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:945:1, end:945:2))"]
#[derive(Clone, Debug)]
struct div1_instructionVar130 {
    M_0t: TableM_0t,
    N_0t: TableN_0t,
}
impl div1_instructionVar130 {
    fn display_extend<T>(
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
            DisplayElement::Literal("div1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 4i128 {
            return None;
        }
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1059:1, end:1059:2))"]
#[derive(Clone, Debug)]
struct dmuls_l_instructionVar131 {
    M_0t: TableM_0t,
    N_0t: TableN_0t,
}
impl dmuls_l_instructionVar131 {
    fn display_extend<T>(
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
            DisplayElement::Literal("dmuls.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 13i128 {
            return None;
        }
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1070:1, end:1070:2))"]
#[derive(Clone, Debug)]
struct dmulu_l_instructionVar132 {
    M_0t: TableM_0t,
    N_0t: TableN_0t,
}
impl dmulu_l_instructionVar132 {
    fn display_extend<T>(
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
            DisplayElement::Literal("dmulu.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 5i128 {
            return None;
        }
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1091:1, end:1091:2))"]
#[derive(Clone, Debug)]
struct exts_b_instructionVar133 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl exts_b_instructionVar133 {
    fn display_extend<T>(
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
            DisplayElement::Literal("exts.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 14i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1100:1, end:1100:2))"]
#[derive(Clone, Debug)]
struct exts_w_instructionVar134 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl exts_w_instructionVar134 {
    fn display_extend<T>(
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
            DisplayElement::Literal("exts.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 15i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1110:1, end:1110:2))"]
#[derive(Clone, Debug)]
struct extu_b_instructionVar135 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl extu_b_instructionVar135 {
    fn display_extend<T>(
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
            DisplayElement::Literal("extu.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 12i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1120:1, end:1120:2))"]
#[derive(Clone, Debug)]
struct extu_w_instructionVar136 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl extu_w_instructionVar136 {
    fn display_extend<T>(
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
            DisplayElement::Literal("extu.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 13i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1145:1, end:1145:2))"]
#[derive(Clone, Debug)]
struct fadd_instructionVar137 {
    FRM_0: TokenField_FRM_0,
    FRN_0: TokenField_FRN_0,
}
impl fadd_instructionVar137 {
    fn display_extend<T>(
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
            self.FRM_0.display(),
            DisplayElement::Literal(","),
            self.FRN_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 0i128 {
            return None;
        }
        let FRN_0 = token_parser.TokenFieldFRN_0();
        let DRN_1 = token_parser.TokenFieldDRN_1();
        let FRM_0 = token_parser.TokenFieldFRM_0();
        let DRM_1 = token_parser.TokenFieldDRM_1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRN_0, FRM_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1160:1, end:1160:2))"]
#[derive(Clone, Debug)]
struct fcmp_instructionVar138 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
    FRM_0t: TableFRM_0t,
    DRM_1t: TableDRM_1t,
}
impl fcmp_instructionVar138 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fcmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRM_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 4i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRM_0t = if let Some((len, table)) = TableFRM_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRM_1t = if let Some((len, table)) = TableDRM_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let OP_6 = token_parser.TokenFieldOP_6();
        let OP_10 = token_parser.TokenFieldOP_10();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                FRN_0t,
                DRN_1t,
                FRM_0t,
                DRM_1t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1174:1, end:1174:2))"]
#[derive(Clone, Debug)]
struct fcmp_instructionVar139 {
    FRM_0t: TableFRM_0t,
    DRM_1t: TableDRM_1t,
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
}
impl fcmp_instructionVar139 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fcmp"),
            DisplayElement::Literal("/gt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRM_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 5i128 {
            return None;
        }
        let FRM_0t = if let Some((len, table)) = TableFRM_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRM_1t = if let Some((len, table)) = TableDRM_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
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
        Some((
            pattern_len,
            Self {
                FRM_0t,
                DRM_1t,
                FRN_0t,
                DRN_1t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1216:1, end:1216:2))"]
#[derive(Clone, Debug)]
struct fdiv_instructionVar140 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
    FRM_0t: TableFRM_0t,
    DRM_1t: TableDRM_1t,
}
impl fdiv_instructionVar140 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fdiv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRM_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 3i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRM_0t = if let Some((len, table)) = TableFRM_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRM_1t = if let Some((len, table)) = TableDRM_1t::parse(
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
        Some((
            pattern_len,
            Self {
                FRN_0t,
                DRN_1t,
                FRM_0t,
                DRM_1t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1303:1, end:1303:2))"]
#[derive(Clone, Debug)]
struct fmac_instructionVar141 {
    FRN_0t: TableFRN_0t,
    FRM_0t: TableFRM_0t,
    FR0_t: TableFR0_t,
}
impl fmac_instructionVar141 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmac"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FR0_t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FRM_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 14i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRM_0t = if let Some((len, table)) = TableFRM_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FR0_t = if let Some((len, table)) =
            TableFR0_t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                FRN_0t,
                FRM_0t,
                FR0_t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1318:1, end:1318:2))"]
#[derive(Clone, Debug)]
struct fmov_instructionVar142 {
    FRM_0: TokenField_FRM_0,
    FRN_0: TokenField_FRN_0,
}
impl fmov_instructionVar142 {
    fn display_extend<T>(
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
            self.FRM_0.display(),
            DisplayElement::Literal(","),
            self.FRN_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 12i128 {
            return None;
        }
        let FRN_0 = token_parser.TokenFieldFRN_0();
        let XDRN = token_parser.TokenFieldXDRN();
        let FRM_0 = token_parser.TokenFieldFRM_0();
        let XDRM = token_parser.TokenFieldXDRM();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRN_0, FRM_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1332:1, end:1332:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar143 {
    FRN_0: TokenField_FRN_0,
    M_0t_at1: TableM_0t_at1,
}
impl fmov_s_instructionVar143 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.FRN_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 8i128 {
            return None;
        }
        let M_0t_at1 = if let Some((len, table)) = TableM_0t_at1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let XDRN = token_parser.TokenFieldXDRN();
        let FRN_0 = token_parser.TokenFieldFRN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { M_0t_at1, FRN_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1350:1, end:1350:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar144 {
    FRM_0: TokenField_FRM_0,
    N_0t_at1: TableN_0t_at1,
}
impl fmov_s_instructionVar144 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            self.FRM_0.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 10i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRM_0 = token_parser.TokenFieldFRM_0();
        let XDRM = token_parser.TokenFieldXDRM();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0t_at1, FRM_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1365:1, end:1365:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar145 {
    FRN_0: TokenField_FRN_0,
    M_0t_at: TableM_0t_at,
}
impl fmov_s_instructionVar145 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.FRN_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 9i128 {
            return None;
        }
        let M_0t_at = if let Some((len, table)) = TableM_0t_at::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let XDRN = token_parser.TokenFieldXDRN();
        let FRN_0 = token_parser.TokenFieldFRN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { M_0t_at, FRN_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1382:1, end:1382:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar146 {
    FRM_0: TokenField_FRM_0,
    N_0t_at_neg: TableN_0t_at_neg,
}
impl fmov_s_instructionVar146 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            self.FRM_0.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 11i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let XDRM = token_parser.TokenFieldXDRM();
        let FRM_0 = token_parser.TokenFieldFRM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0t_at_neg, FRM_0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1399:1, end:1399:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar147 {
    FRN_0: TokenField_FRN_0,
    M_0t_at_with_r0: TableM_0t_at_with_r0,
}
impl fmov_s_instructionVar147 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), self.FRN_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 6i128 {
            return None;
        }
        let M_0t_at_with_r0 = if let Some((len, table)) =
            TableM_0t_at_with_r0::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let XDRN = token_parser.TokenFieldXDRN();
        let FRN_0 = token_parser.TokenFieldFRN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                M_0t_at_with_r0,
                FRN_0,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1414:1, end:1414:2))"]
#[derive(Clone, Debug)]
struct fmov_instructionVar148 {
    FRM_0: TokenField_FRM_0,
    N_0t_at_with_r0: TableN_0t_at_with_r0,
}
impl fmov_instructionVar148 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmov"),
            DisplayElement::Literal(" "),
            self.FRM_0.display(),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.N_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 7i128 {
            return None;
        }
        let N_0t_at_with_r0 = if let Some((len, table)) =
            TableN_0t_at_with_r0::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let XDRM = token_parser.TokenFieldXDRM();
        let FRM_0 = token_parser.TokenFieldFRM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                N_0t_at_with_r0,
                FRM_0,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1429:1, end:1429:2))"]
#[derive(Clone, Debug)]
struct fmul_instructionVar149 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
    FRM_0t: TableFRM_0t,
    DRM_1t: TableDRM_1t,
}
impl fmul_instructionVar149 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fmul"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRM_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 2i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRM_0t = if let Some((len, table)) = TableFRM_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRM_1t = if let Some((len, table)) = TableDRM_1t::parse(
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
        Some((
            pattern_len,
            Self {
                FRN_0t,
                DRN_1t,
                FRM_0t,
                DRM_1t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1555:1, end:1555:2))"]
#[derive(Clone, Debug)]
struct fsub_instructionVar150 {
    FRN_0t: TableFRN_0t,
    DRN_1t: TableDRN_1t,
    FRM_0t: TableFRM_0t,
    DRM_1t: TableDRM_1t,
}
impl fsub_instructionVar150 {
    fn display_extend<T>(
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
            DisplayElement::Literal("fsub"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.FRM_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.FRN_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 1i128 {
            return None;
        }
        let FRN_0t = if let Some((len, table)) = TableFRN_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRN_1t = if let Some((len, table)) = TableDRN_1t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let FRM_0t = if let Some((len, table)) = TableFRM_0t::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let DRM_1t = if let Some((len, table)) = TableDRM_1t::parse(
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
        Some((
            pattern_len,
            Self {
                FRN_0t,
                DRN_1t,
                FRM_0t,
                DRM_1t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1916:1, end:1916:2))"]
#[derive(Clone, Debug)]
struct mac_l_instructionVar151 {
    M_0t_at: TableM_0t_at,
    N_0t_at: TableN_0t_at,
}
impl mac_l_instructionVar151 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mac.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 15i128 {
            return None;
        }
        let M_0t_at = if let Some((len, table)) = TableM_0t_at::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let N_0t_at = if let Some((len, table)) = TableN_0t_at::parse(
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
        Some((pattern_len, Self { M_0t_at, N_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1938:1, end:1938:2))"]
#[derive(Clone, Debug)]
struct mac_w_instructionVar152 {
    M_0t_at: TableM_0t_at,
    N_0t_at: TableN_0t_at,
}
impl mac_w_instructionVar152 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mac.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 15i128 {
            return None;
        }
        let M_0t_at = if let Some((len, table)) = TableM_0t_at::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let N_0t_at = if let Some((len, table)) = TableN_0t_at::parse(
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
        Some((pattern_len, Self { M_0t_at, N_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1959:1, end:1959:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar153 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl mov_instructionVar153 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 3i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1969:1, end:1969:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar154 {
    N_0t_at1: TableN_0t_at1,
    M_0t: TableM_0t,
}
impl mov_b_instructionVar154 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 0i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at1, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1979:1, end:1979:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar155 {
    N_0t_at1: TableN_0t_at1,
    M_0t: TableM_0t,
}
impl mov_w_instructionVar155 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 1i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at1, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1989:1, end:1989:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar156 {
    N_0t_at1: TableN_0t_at1,
    M_0t: TableM_0t,
}
impl mov_l_instructionVar156 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 2i128 {
            return None;
        }
        let N_0t_at1 = if let Some((len, table)) = TableN_0t_at1::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at1, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:1999:1, end:1999:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar157 {
    N_0t: TableN_0t,
    M_0t_at1: TableM_0t_at1,
}
impl mov_b_instructionVar157 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 0i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at1 = if let Some((len, table)) = TableM_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t, M_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2009:1, end:2009:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar158 {
    N_0t: TableN_0t,
    M_0t_at1: TableM_0t_at1,
}
impl mov_w_instructionVar158 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 1i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at1 = if let Some((len, table)) = TableM_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t, M_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2019:1, end:2019:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar159 {
    N_0t: TableN_0t,
    M_0t_at1: TableM_0t_at1,
}
impl mov_l_instructionVar159 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 2i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at1 = if let Some((len, table)) = TableM_0t_at1::parse(
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
        Some((pattern_len, Self { N_0t, M_0t_at1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2029:1, end:2029:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar160 {
    N_0t_at_neg: TableN_0t_at_neg,
    M_0t: TableM_0t,
}
impl mov_b_instructionVar160 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 4i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2040:1, end:2040:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar161 {
    N_0t_at_neg: TableN_0t_at_neg,
    M_0t: TableM_0t,
}
impl mov_w_instructionVar161 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 5i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2051:1, end:2051:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar162 {
    N_0t_at_neg: TableN_0t_at_neg,
    M_0t: TableM_0t,
}
impl mov_l_instructionVar162 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_neg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 6i128 {
            return None;
        }
        let N_0t_at_neg = if let Some((len, table)) = TableN_0t_at_neg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t_at_neg, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2062:1, end:2062:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar163 {
    N_0t: TableN_0t,
    M_0t_at: TableM_0t_at,
}
impl mov_b_instructionVar163 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 4i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at = if let Some((len, table)) = TableM_0t_at::parse(
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
        Some((pattern_len, Self { N_0t, M_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2073:1, end:2073:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar164 {
    N_0t: TableN_0t,
    M_0t_at: TableM_0t_at,
}
impl mov_w_instructionVar164 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 5i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at = if let Some((len, table)) = TableM_0t_at::parse(
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
        Some((pattern_len, Self { N_0t, M_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2084:1, end:2084:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar165 {
    N_0t: TableN_0t,
    M_0t_at: TableM_0t_at,
}
impl mov_l_instructionVar165 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 6i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at = if let Some((len, table)) = TableM_0t_at::parse(
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
        Some((pattern_len, Self { N_0t, M_0t_at }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2095:1, end:2095:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar166 {
    N_0t_at_with_r0: TableN_0t_at_with_r0,
    M_0t: TableM_0t,
}
impl mov_b_instructionVar166 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 4i128 {
            return None;
        }
        let N_0t_at_with_r0 = if let Some((len, table)) =
            TableN_0t_at_with_r0::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                N_0t_at_with_r0,
                M_0t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2105:1, end:2105:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar167 {
    N_0t_at_with_r0: TableN_0t_at_with_r0,
    M_0t: TableM_0t,
}
impl mov_w_instructionVar167 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 5i128 {
            return None;
        }
        let N_0t_at_with_r0 = if let Some((len, table)) =
            TableN_0t_at_with_r0::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                N_0t_at_with_r0,
                M_0t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2115:1, end:2115:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar168 {
    N_0t_at_with_r0: TableN_0t_at_with_r0,
    M_0t: TableM_0t,
}
impl mov_l_instructionVar168 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 6i128 {
            return None;
        }
        let N_0t_at_with_r0 = if let Some((len, table)) =
            TableN_0t_at_with_r0::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                N_0t_at_with_r0,
                M_0t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2125:1, end:2125:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar169 {
    N_0t: TableN_0t,
    M_0t_at_with_r0: TableM_0t_at_with_r0,
}
impl mov_b_instructionVar169 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 12i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at_with_r0 = if let Some((len, table)) =
            TableM_0t_at_with_r0::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t,
                M_0t_at_with_r0,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2135:1, end:2135:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar170 {
    N_0t: TableN_0t,
    M_0t_at_with_r0: TableM_0t_at_with_r0,
}
impl mov_w_instructionVar170 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 13i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at_with_r0 = if let Some((len, table)) =
            TableM_0t_at_with_r0::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t,
                M_0t_at_with_r0,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2145:1, end:2145:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar171 {
    N_0t: TableN_0t,
    M_0t_at_with_r0: TableM_0t_at_with_r0,
}
impl mov_l_instructionVar171 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t_at_with_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 14i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t_at_with_r0 = if let Some((len, table)) =
            TableM_0t_at_with_r0::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t,
                M_0t_at_with_r0,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2185:1, end:2185:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar172 {
    U_0t_gbr_at_1: TableU_0t_gbr_at_1,
}
impl mov_b_instructionVar172 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_gbr_at_1.display_extend(
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
        if token_parser.TokenFieldOP_2().disassembly() != 196i128 {
            return None;
        }
        let U_0t_gbr_at_1 = if let Some((len, table)) =
            TableU_0t_gbr_at_1::parse(
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
        Some((pattern_len, Self { U_0t_gbr_at_1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2195:1, end:2195:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar173 {
    U_0t_gbr_at_2: TableU_0t_gbr_at_2,
}
impl mov_w_instructionVar173 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_gbr_at_2.display_extend(
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
        if token_parser.TokenFieldOP_2().disassembly() != 197i128 {
            return None;
        }
        let U_0t_gbr_at_2 = if let Some((len, table)) =
            TableU_0t_gbr_at_2::parse(
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
        Some((pattern_len, Self { U_0t_gbr_at_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2205:1, end:2205:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar174 {
    U_0t_gbr_at_4: TableU_0t_gbr_at_4,
}
impl mov_l_instructionVar174 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_gbr_at_4.display_extend(
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
        if token_parser.TokenFieldOP_2().disassembly() != 198i128 {
            return None;
        }
        let U_0t_gbr_at_4 = if let Some((len, table)) =
            TableU_0t_gbr_at_4::parse(
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
        Some((pattern_len, Self { U_0t_gbr_at_4 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2215:1, end:2215:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar175 {
    U_0t_gbr_at_1: TableU_0t_gbr_at_1,
}
impl mov_b_instructionVar175 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_gbr_at_1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 192i128 {
            return None;
        }
        let U_0t_gbr_at_1 = if let Some((len, table)) =
            TableU_0t_gbr_at_1::parse(
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
        Some((pattern_len, Self { U_0t_gbr_at_1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2225:1, end:2225:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar176 {
    U_0t_gbr_at_2: TableU_0t_gbr_at_2,
}
impl mov_w_instructionVar176 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_gbr_at_2.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 193i128 {
            return None;
        }
        let U_0t_gbr_at_2 = if let Some((len, table)) =
            TableU_0t_gbr_at_2::parse(
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
        Some((pattern_len, Self { U_0t_gbr_at_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2235:1, end:2235:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar177 {
    U_0t_gbr_at_4: TableU_0t_gbr_at_4,
}
impl mov_l_instructionVar177 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_gbr_at_4.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 194i128 {
            return None;
        }
        let U_0t_gbr_at_4 = if let Some((len, table)) =
            TableU_0t_gbr_at_4::parse(
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
        Some((pattern_len, Self { U_0t_gbr_at_4 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2245:1, end:2245:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar178 {
    M_0t: TableM_0t,
    U_2t_M0_dispr01: TableU_2t_M0_dispr01,
}
impl mov_b_instructionVar178 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.U_2t_M0_dispr01.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 128i128 {
            return None;
        }
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let U_2t_M0_dispr01 = if let Some((len, table)) =
            TableU_2t_M0_dispr01::parse(
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
        Some((
            pattern_len,
            Self {
                M_0t,
                U_2t_M0_dispr01,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2255:1, end:2255:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar179 {
    M_0t: TableM_0t,
    U_2t_M0_dispr02: TableU_2t_M0_dispr02,
}
impl mov_w_instructionVar179 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.U_2t_M0_dispr02.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 129i128 {
            return None;
        }
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let U_2t_M0_dispr02 = if let Some((len, table)) =
            TableU_2t_M0_dispr02::parse(
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
        Some((
            pattern_len,
            Self {
                M_0t,
                U_2t_M0_dispr02,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2275:1, end:2275:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar180 {
    M_0t: TableM_0t,
    U_2t_M0_dispr01: TableU_2t_M0_dispr01,
}
impl mov_b_instructionVar180 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_2t_M0_dispr01.display_extend(
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
        if token_parser.TokenFieldOP_2().disassembly() != 132i128 {
            return None;
        }
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let U_2t_M0_dispr01 = if let Some((len, table)) =
            TableU_2t_M0_dispr01::parse(
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
        Some((
            pattern_len,
            Self {
                M_0t,
                U_2t_M0_dispr01,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2285:1, end:2285:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar181 {
    U_2t_M0_dispr02: TableU_2t_M0_dispr02,
    M_0t: TableM_0t,
}
impl mov_w_instructionVar181 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_2t_M0_dispr02.display_extend(
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
        if token_parser.TokenFieldOP_2().disassembly() != 133i128 {
            return None;
        }
        let U_2t_M0_dispr02 = if let Some((len, table)) =
            TableU_2t_M0_dispr02::parse(
                tokens_current,
                &mut context_instance,
                inst_start,
            ) {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((
            pattern_len,
            Self {
                U_2t_M0_dispr02,
                M_0t,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2305:1, end:2305:2))"]
#[derive(Clone, Debug)]
struct mova_instructionVar182 {
    U_0t_4pc: TableU_0t_4pc,
}
impl mova_instructionVar182 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_4pc.display_extend(
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
        if token_parser.TokenFieldOP_2().disassembly() != 199i128 {
            return None;
        }
        let U_0t_4pc = if let Some((len, table)) = TableU_0t_4pc::parse(
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
        Some((pattern_len, Self { U_0t_4pc }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2354:1, end:2354:2))"]
#[derive(Clone, Debug)]
struct mul_l_instructionVar183 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl mul_l_instructionVar183 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mul.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 7i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2364:1, end:2364:2))"]
#[derive(Clone, Debug)]
struct muls_w_instructionVar184 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl muls_w_instructionVar184 {
    fn display_extend<T>(
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
            DisplayElement::Literal("muls.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 15i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2374:1, end:2374:2))"]
#[derive(Clone, Debug)]
struct mulu_w_instructionVar185 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl mulu_w_instructionVar185 {
    fn display_extend<T>(
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
            DisplayElement::Literal("mulu.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 14i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2384:1, end:2384:2))"]
#[derive(Clone, Debug)]
struct neg_instructionVar186 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl neg_instructionVar186 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("neg"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 11i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2394:1, end:2394:2))"]
#[derive(Clone, Debug)]
struct negc_instructionVar187 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl negc_instructionVar187 {
    fn display_extend<T>(
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
            DisplayElement::Literal("negc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 10i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2414:1, end:2414:2))"]
#[derive(Clone, Debug)]
struct not_instructionVar188 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl not_instructionVar188 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 7i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2449:1, end:2449:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar189 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl or_instructionVar189 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 11i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2459:1, end:2459:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar190 {
    U_0t_r0: TableU_0t_r0,
}
impl or_instructionVar190 {
    fn display_extend<T>(
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
        self.U_0t_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 203i128 {
            return None;
        }
        let U_0t_r0 = if let Some((len, table)) = TableU_0t_r0::parse(
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
        Some((pattern_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2469:1, end:2469:2))"]
#[derive(Clone, Debug)]
struct or_b_instructionVar191 {
    U_0t1: TableU_0t1,
}
impl or_b_instructionVar191 {
    fn display_extend<T>(
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
            DisplayElement::Literal("or.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 207i128 {
            return None;
        }
        let U_0t1 = if let Some((len, table)) =
            TableU_0t1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2575:1, end:2575:2))"]
#[derive(Clone, Debug)]
struct shad_instructionVar192 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl shad_instructionVar192 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shad"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 12i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2611:1, end:2611:2))"]
#[derive(Clone, Debug)]
struct shld_instructionVar193 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl shld_instructionVar193 {
    fn display_extend<T>(
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
            DisplayElement::Literal("shld"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 13i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2984:1, end:2984:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar194 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl sub_instructionVar194 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 8i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2994:1, end:2994:2))"]
#[derive(Clone, Debug)]
struct subc_instructionVar195 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl subc_instructionVar195 {
    fn display_extend<T>(
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
            DisplayElement::Literal("subc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 10i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3007:1, end:3007:2))"]
#[derive(Clone, Debug)]
struct subv_instructionVar196 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl subv_instructionVar196 {
    fn display_extend<T>(
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
            DisplayElement::Literal("subv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 11i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3017:1, end:3017:2))"]
#[derive(Clone, Debug)]
struct swap_b_instructionVar197 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl swap_b_instructionVar197 {
    fn display_extend<T>(
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
            DisplayElement::Literal("swap.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 8i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3029:1, end:3029:2))"]
#[derive(Clone, Debug)]
struct swap_w_instructionVar198 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl swap_w_instructionVar198 {
    fn display_extend<T>(
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
            DisplayElement::Literal("swap.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 9i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3063:1, end:3063:2))"]
#[derive(Clone, Debug)]
struct trapa_instructionVar199 {
    U_0t: TableU_0t,
}
impl trapa_instructionVar199 {
    fn display_extend<T>(
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
            DisplayElement::Literal("trapa"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 195i128 {
            return None;
        }
        let U_0t = if let Some((len, table)) =
            TableU_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { U_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3070:1, end:3070:2))"]
#[derive(Clone, Debug)]
struct tst_instructionVar200 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl tst_instructionVar200 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 8i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3080:1, end:3080:2))"]
#[derive(Clone, Debug)]
struct tst_instructionVar201 {
    U_0t_r0: TableU_0t_r0,
}
impl tst_instructionVar201 {
    fn display_extend<T>(
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
        self.U_0t_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 200i128 {
            return None;
        }
        let U_0t_r0 = if let Some((len, table)) = TableU_0t_r0::parse(
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
        Some((pattern_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3090:1, end:3090:2))"]
#[derive(Clone, Debug)]
struct tst_b_instructionVar202 {
    U_0t1: TableU_0t1,
}
impl tst_b_instructionVar202 {
    fn display_extend<T>(
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
            DisplayElement::Literal("tst.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 204i128 {
            return None;
        }
        let U_0t1 = if let Some((len, table)) =
            TableU_0t1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3099:1, end:3099:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar203 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl xor_instructionVar203 {
    fn display_extend<T>(
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
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 10i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3109:1, end:3109:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar204 {
    U_0t_r0: TableU_0t_r0,
}
impl xor_instructionVar204 {
    fn display_extend<T>(
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
        self.U_0t_r0.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 202i128 {
            return None;
        }
        let U_0t_r0 = if let Some((len, table)) = TableU_0t_r0::parse(
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
        Some((pattern_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3119:1, end:3119:2))"]
#[derive(Clone, Debug)]
struct xor_b_instructionVar205 {
    U_0t1: TableU_0t1,
}
impl xor_b_instructionVar205 {
    fn display_extend<T>(
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
            DisplayElement::Literal("xor.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_2().disassembly() != 206i128 {
            return None;
        }
        let U_0t1 = if let Some((len, table)) =
            TableU_0t1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:3129:1, end:3129:2))"]
#[derive(Clone, Debug)]
struct xtrct_instructionVar206 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
}
impl xtrct_instructionVar206 {
    fn display_extend<T>(
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
            DisplayElement::Literal("xtrct"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldOP_1().disassembly() != 13i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:644:1, end:644:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar207 {
    N_0t: TableN_0t,
    I_0t: TableI_0t,
}
impl add_instructionVar207 {
    fn display_extend<T>(
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
        self.I_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 7i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let I_0t = if let Some((len, table)) =
            TableI_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, I_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:728:1, end:728:2))"]
#[derive(Clone, Debug)]
struct bra_instructionVar208 {
    I_1tbranch: TableI_1tbranch,
}
impl bra_instructionVar208 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("bra"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.I_1tbranch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 10i128 {
            return None;
        }
        let I_1tbranch = if let Some((len, table)) = TableI_1tbranch::parse(
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
        Some((pattern_len, Self { I_1tbranch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:751:1, end:751:2))"]
#[derive(Clone, Debug)]
struct bsr_instructionVar209 {
    I_1tbranch: TableI_1tbranch,
}
impl bsr_instructionVar209 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("bsr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.I_1tbranch.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 11i128 {
            return None;
        }
        let I_1tbranch = if let Some((len, table)) = TableI_1tbranch::parse(
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
        Some((pattern_len, Self { I_1tbranch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2155:1, end:2155:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar210 {
    N_0t: TableN_0t,
    I_0t: TableI_0t,
}
impl mov_instructionVar210 {
    fn display_extend<T>(
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
        self.I_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 14i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let I_0t = if let Some((len, table)) =
            TableI_0t::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { N_0t, I_0t }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2165:1, end:2165:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar211 {
    N_0t: TableN_0t,
    U_0t_2pc: TableU_0t_2pc,
}
impl mov_w_instructionVar211 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_2pc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 9i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let U_0t_2pc = if let Some((len, table)) = TableU_0t_2pc::parse(
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
        Some((pattern_len, Self { N_0t, U_0t_2pc }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2175:1, end:2175:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar212 {
    N_0t: TableN_0t,
    U_0t_4pc: TableU_0t_4pc,
}
impl mov_l_instructionVar212 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_0t_4pc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 13i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let U_0t_4pc = if let Some((len, table)) = TableU_0t_4pc::parse(
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
        Some((pattern_len, Self { N_0t, U_0t_4pc }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2265:1, end:2265:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar213 {
    M_0t: TableM_0t,
    N_0t: TableN_0t,
    U_2t_N0_dispr04: TableU_2t_N0_dispr04,
}
impl mov_l_instructionVar213 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.M_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.U_2t_N0_dispr04.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 1i128 {
            return None;
        }
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let U_2t_N0_dispr04 = if let Some((len, table)) =
            TableU_2t_N0_dispr04::parse(
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
        Some((
            pattern_len,
            Self {
                M_0t,
                N_0t,
                U_2t_N0_dispr04,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:2295:1, end:2295:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar214 {
    N_0t: TableN_0t,
    M_0t: TableM_0t,
    U_2t_M0_dispr04: TableU_2t_M0_dispr04,
}
impl mov_l_instructionVar214 {
    fn display_extend<T>(
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
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.U_2t_M0_dispr04.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        self.N_0t.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldOP_0().disassembly() != 5i128 {
            return None;
        }
        let N_0t = if let Some((len, table)) =
            TableN_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let M_0t = if let Some((len, table)) =
            TableM_0t::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u32);
            table
        } else {
            return None;
        };
        let U_2t_M0_dispr04 = if let Some((len, table)) =
            TableU_2t_M0_dispr04::parse(
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
        Some((
            pattern_len,
            Self {
                N_0t,
                M_0t,
                U_2t_M0_dispr04,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(clrmac_instructionVar0),
    Var1(clrs_instructionVar1),
    Var2(clrt_instructionVar2),
    Var3(div0u_instructionVar3),
    Var4(frchg_instructionVar4),
    Var5(fschg_instructionVar5),
    Var6(fpchg_instructionVar6),
    Var7(ldtlb_instructionVar7),
    Var8(movua_l_instructionVar8),
    Var9(nop_instructionVar9),
    Var10(rte_instructionVar10),
    Var11(rts_instructionVar11),
    Var12(sets_instructionVar12),
    Var13(sett_instructionVar13),
    Var14(sleep_instructionVar14),
    Var15(synco_instructionVar15),
    Var16(ftrv_instructionVar16),
    Var17(fcnvds_instructionVar17),
    Var18(fcnvsd_instructionVar18),
    Var19(braf_instructionVar19),
    Var20(bsrf_instructionVar20),
    Var21(cmp_instructionVar21),
    Var22(cmp_instructionVar22),
    Var23(dt_instructionVar23),
    Var24(fabs_instructionVar24),
    Var25(fipr_instructionVar25),
    Var26(fldi0_instructionVar26),
    Var27(fldi1_instructionVar27),
    Var28(flds_instructionVar28),
    Var29(float_instructionVar29),
    Var30(fneg_instructionVar30),
    Var31(fsqrt_instructionVar31),
    Var32(fsrra_instructionVar32),
    Var33(fsca_instructionVar33),
    Var34(fsts_instructionVar34),
    Var35(ftrc_instructionVar35),
    Var36(icbi_instructionVar36),
    Var37(jmp_instructionVar37),
    Var38(jsr_instructionVar38),
    Var39(ldc_instructionVar39),
    Var40(ldc_instructionVar40),
    Var41(ldc_instructionVar41),
    Var42(ldc_instructionVar42),
    Var43(ldc_instructionVar43),
    Var44(ldc_instructionVar44),
    Var45(ldc_l_instructionVar45),
    Var46(ldc_l_instructionVar46),
    Var47(ldc_l_instructionVar47),
    Var48(ldc_l_instructionVar48),
    Var49(ldc_l_instructionVar49),
    Var50(ldc_l_instructionVar50),
    Var51(lds_instructionVar51),
    Var52(lds_l_instructionVar52),
    Var53(lds_instructionVar53),
    Var54(lds_l_instructionVar54),
    Var55(lds_instructionVar55),
    Var56(lds_instructionVar56),
    Var57(lds_instructionVar57),
    Var58(lds_l_instructionVar58),
    Var59(lds_l_instructionVar59),
    Var60(lds_l_instructionVar60),
    Var61(movca_l_instructionVar61),
    Var62(movt_instructionVar62),
    Var63(movua_l_instructionVar63),
    Var64(movua_l_instructionVar64),
    Var65(ocbi_instructionVar65),
    Var66(ocbp_instructionVar66),
    Var67(ocbwb_instructionVar67),
    Var68(pref_instructionVar68),
    Var69(rotcl_instructionVar69),
    Var70(rotcr_instructionVar70),
    Var71(rotl_instructionVar71),
    Var72(rotr_instructionVar72),
    Var73(shal_instructionVar73),
    Var74(shar_instructionVar74),
    Var75(shll_instructionVar75),
    Var76(shll2_instructionVar76),
    Var77(shll8_instructionVar77),
    Var78(shll16_instructionVar78),
    Var79(shlr_instructionVar79),
    Var80(shlr2_instructionVar80),
    Var81(shlr8_instructionVar81),
    Var82(shlr16_instructionVar82),
    Var83(stc_instructionVar83),
    Var84(stc_instructionVar84),
    Var85(stc_instructionVar85),
    Var86(stc_instructionVar86),
    Var87(stc_instructionVar87),
    Var88(stc_instructionVar88),
    Var89(stc_instructionVar89),
    Var90(stc_l_instructionVar90),
    Var91(stc_l_instructionVar91),
    Var92(stc_l_instructionVar92),
    Var93(stc_l_instructionVar93),
    Var94(stc_l_instructionVar94),
    Var95(stc_l_instructionVar95),
    Var96(stc_l_instructionVar96),
    Var97(sts_instructionVar97),
    Var98(sts_instructionVar98),
    Var99(sts_instructionVar99),
    Var100(sts_l_instructionVar100),
    Var101(sts_l_instructionVar101),
    Var102(sts_l_instructionVar102),
    Var103(sts_instructionVar103),
    Var104(sts_instructionVar104),
    Var105(sts_l_instructionVar105),
    Var106(sts_l_instructionVar106),
    Var107(tas_b_instructionVar107),
    Var108(ldc_instructionVar108),
    Var109(ldc_l_instructionVar109),
    Var110(stc_instructionVar110),
    Var111(stc_l_instructionVar111),
    Var112(add_instructionVar112),
    Var113(addc_instructionVar113),
    Var114(addv_instructionVar114),
    Var115(and_instructionVar115),
    Var116(and_instructionVar116),
    Var117(and_b_instructionVar117),
    Var118(bf_instructionVar118),
    Var119(bf_instructionVar119),
    Var120(bt_instructionVar120),
    Var121(bt_instructionVar121),
    Var122(cmp_instructionVar122),
    Var123(cmp_instructionVar123),
    Var124(cmp_instructionVar124),
    Var125(cmp_instructionVar125),
    Var126(cmp_instructionVar126),
    Var127(cmp_instructionVar127),
    Var128(cmp_instructionVar128),
    Var129(div0s_instructionVar129),
    Var130(div1_instructionVar130),
    Var131(dmuls_l_instructionVar131),
    Var132(dmulu_l_instructionVar132),
    Var133(exts_b_instructionVar133),
    Var134(exts_w_instructionVar134),
    Var135(extu_b_instructionVar135),
    Var136(extu_w_instructionVar136),
    Var137(fadd_instructionVar137),
    Var138(fcmp_instructionVar138),
    Var139(fcmp_instructionVar139),
    Var140(fdiv_instructionVar140),
    Var141(fmac_instructionVar141),
    Var142(fmov_instructionVar142),
    Var143(fmov_s_instructionVar143),
    Var144(fmov_s_instructionVar144),
    Var145(fmov_s_instructionVar145),
    Var146(fmov_s_instructionVar146),
    Var147(fmov_s_instructionVar147),
    Var148(fmov_instructionVar148),
    Var149(fmul_instructionVar149),
    Var150(fsub_instructionVar150),
    Var151(mac_l_instructionVar151),
    Var152(mac_w_instructionVar152),
    Var153(mov_instructionVar153),
    Var154(mov_b_instructionVar154),
    Var155(mov_w_instructionVar155),
    Var156(mov_l_instructionVar156),
    Var157(mov_b_instructionVar157),
    Var158(mov_w_instructionVar158),
    Var159(mov_l_instructionVar159),
    Var160(mov_b_instructionVar160),
    Var161(mov_w_instructionVar161),
    Var162(mov_l_instructionVar162),
    Var163(mov_b_instructionVar163),
    Var164(mov_w_instructionVar164),
    Var165(mov_l_instructionVar165),
    Var166(mov_b_instructionVar166),
    Var167(mov_w_instructionVar167),
    Var168(mov_l_instructionVar168),
    Var169(mov_b_instructionVar169),
    Var170(mov_w_instructionVar170),
    Var171(mov_l_instructionVar171),
    Var172(mov_b_instructionVar172),
    Var173(mov_w_instructionVar173),
    Var174(mov_l_instructionVar174),
    Var175(mov_b_instructionVar175),
    Var176(mov_w_instructionVar176),
    Var177(mov_l_instructionVar177),
    Var178(mov_b_instructionVar178),
    Var179(mov_w_instructionVar179),
    Var180(mov_b_instructionVar180),
    Var181(mov_w_instructionVar181),
    Var182(mova_instructionVar182),
    Var183(mul_l_instructionVar183),
    Var184(muls_w_instructionVar184),
    Var185(mulu_w_instructionVar185),
    Var186(neg_instructionVar186),
    Var187(negc_instructionVar187),
    Var188(not_instructionVar188),
    Var189(or_instructionVar189),
    Var190(or_instructionVar190),
    Var191(or_b_instructionVar191),
    Var192(shad_instructionVar192),
    Var193(shld_instructionVar193),
    Var194(sub_instructionVar194),
    Var195(subc_instructionVar195),
    Var196(subv_instructionVar196),
    Var197(swap_b_instructionVar197),
    Var198(swap_w_instructionVar198),
    Var199(trapa_instructionVar199),
    Var200(tst_instructionVar200),
    Var201(tst_instructionVar201),
    Var202(tst_b_instructionVar202),
    Var203(xor_instructionVar203),
    Var204(xor_instructionVar204),
    Var205(xor_b_instructionVar205),
    Var206(xtrct_instructionVar206),
    Var207(add_instructionVar207),
    Var208(bra_instructionVar208),
    Var209(bsr_instructionVar209),
    Var210(mov_instructionVar210),
    Var211(mov_w_instructionVar211),
    Var212(mov_l_instructionVar212),
    Var213(mov_l_instructionVar213),
    Var214(mov_l_instructionVar214),
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
        if let Some((inst_len, parsed)) = clrmac_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = clrs_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = clrt_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = div0u_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = frchg_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = fschg_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = fpchg_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = ldtlb_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = movua_l_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = nop_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = rte_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = rts_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = sets_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = sett_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = sleep_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = synco_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = ftrv_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = fcnvds_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = fcnvsd_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = braf_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = bsrf_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = dt_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = fabs_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = fipr_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = fldi0_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = fldi1_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = flds_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = float_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = fneg_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = fsqrt_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = fsrra_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = fsca_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = fsts_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = ftrc_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = icbi_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = jmp_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = jsr_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_l_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_l_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_l_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_l_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_l_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_l_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_l_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_l_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_l_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_l_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = lds_l_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = movca_l_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = movt_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = movua_l_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = movua_l_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = ocbi_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = ocbp_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = ocbwb_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = pref_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = rotcl_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = rotcr_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = rotl_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = rotr_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = shal_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = shar_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = shll_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = shll2_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = shll8_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = shll16_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = shlr_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = shlr2_instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = shlr8_instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = shlr16_instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var93(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var94(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var95(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var96(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var97(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_l_instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_l_instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_l_instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_l_instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = sts_l_instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = tas_b_instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = ldc_l_instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = stc_l_instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = add_instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = addc_instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = addv_instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = and_instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = and_instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = and_b_instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = bf_instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = bf_instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = bt_instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = bt_instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var122(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var123(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var124(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var125(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var126(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var127(parsed)));
        }
        if let Some((inst_len, parsed)) = cmp_instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var128(parsed)));
        }
        if let Some((inst_len, parsed)) = div0s_instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var129(parsed)));
        }
        if let Some((inst_len, parsed)) = div1_instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var130(parsed)));
        }
        if let Some((inst_len, parsed)) = dmuls_l_instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var131(parsed)));
        }
        if let Some((inst_len, parsed)) = dmulu_l_instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var132(parsed)));
        }
        if let Some((inst_len, parsed)) = exts_b_instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var133(parsed)));
        }
        if let Some((inst_len, parsed)) = exts_w_instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var134(parsed)));
        }
        if let Some((inst_len, parsed)) = extu_b_instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var135(parsed)));
        }
        if let Some((inst_len, parsed)) = extu_w_instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var136(parsed)));
        }
        if let Some((inst_len, parsed)) = fadd_instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var137(parsed)));
        }
        if let Some((inst_len, parsed)) = fcmp_instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var138(parsed)));
        }
        if let Some((inst_len, parsed)) = fcmp_instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var139(parsed)));
        }
        if let Some((inst_len, parsed)) = fdiv_instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var140(parsed)));
        }
        if let Some((inst_len, parsed)) = fmac_instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var141(parsed)));
        }
        if let Some((inst_len, parsed)) = fmov_instructionVar142::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var142(parsed)));
        }
        if let Some((inst_len, parsed)) = fmov_s_instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var143(parsed)));
        }
        if let Some((inst_len, parsed)) = fmov_s_instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var144(parsed)));
        }
        if let Some((inst_len, parsed)) = fmov_s_instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var145(parsed)));
        }
        if let Some((inst_len, parsed)) = fmov_s_instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var146(parsed)));
        }
        if let Some((inst_len, parsed)) = fmov_s_instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var147(parsed)));
        }
        if let Some((inst_len, parsed)) = fmov_instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var148(parsed)));
        }
        if let Some((inst_len, parsed)) = fmul_instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var149(parsed)));
        }
        if let Some((inst_len, parsed)) = fsub_instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var150(parsed)));
        }
        if let Some((inst_len, parsed)) = mac_l_instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var151(parsed)));
        }
        if let Some((inst_len, parsed)) = mac_w_instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var152(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var153(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var154(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var155(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var156(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var157(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var158(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var159(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var160(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var161(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var162(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var163(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar164::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var164(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar165::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var165(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar166::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var166(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar167::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var167(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar168::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var168(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar169::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var169(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar170::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var170(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar171::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var171(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar172::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var172(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar173::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var173(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar174::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var174(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar175::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var175(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar176::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var176(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar177::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var177(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar178::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var178(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar179::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var179(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_b_instructionVar180::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var180(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar181::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var181(parsed)));
        }
        if let Some((inst_len, parsed)) = mova_instructionVar182::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var182(parsed)));
        }
        if let Some((inst_len, parsed)) = mul_l_instructionVar183::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var183(parsed)));
        }
        if let Some((inst_len, parsed)) = muls_w_instructionVar184::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var184(parsed)));
        }
        if let Some((inst_len, parsed)) = mulu_w_instructionVar185::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var185(parsed)));
        }
        if let Some((inst_len, parsed)) = neg_instructionVar186::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var186(parsed)));
        }
        if let Some((inst_len, parsed)) = negc_instructionVar187::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var187(parsed)));
        }
        if let Some((inst_len, parsed)) = not_instructionVar188::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var188(parsed)));
        }
        if let Some((inst_len, parsed)) = or_instructionVar189::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var189(parsed)));
        }
        if let Some((inst_len, parsed)) = or_instructionVar190::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var190(parsed)));
        }
        if let Some((inst_len, parsed)) = or_b_instructionVar191::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var191(parsed)));
        }
        if let Some((inst_len, parsed)) = shad_instructionVar192::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var192(parsed)));
        }
        if let Some((inst_len, parsed)) = shld_instructionVar193::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var193(parsed)));
        }
        if let Some((inst_len, parsed)) = sub_instructionVar194::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var194(parsed)));
        }
        if let Some((inst_len, parsed)) = subc_instructionVar195::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var195(parsed)));
        }
        if let Some((inst_len, parsed)) = subv_instructionVar196::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var196(parsed)));
        }
        if let Some((inst_len, parsed)) = swap_b_instructionVar197::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var197(parsed)));
        }
        if let Some((inst_len, parsed)) = swap_w_instructionVar198::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var198(parsed)));
        }
        if let Some((inst_len, parsed)) = trapa_instructionVar199::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var199(parsed)));
        }
        if let Some((inst_len, parsed)) = tst_instructionVar200::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var200(parsed)));
        }
        if let Some((inst_len, parsed)) = tst_instructionVar201::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var201(parsed)));
        }
        if let Some((inst_len, parsed)) = tst_b_instructionVar202::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var202(parsed)));
        }
        if let Some((inst_len, parsed)) = xor_instructionVar203::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var203(parsed)));
        }
        if let Some((inst_len, parsed)) = xor_instructionVar204::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var204(parsed)));
        }
        if let Some((inst_len, parsed)) = xor_b_instructionVar205::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var205(parsed)));
        }
        if let Some((inst_len, parsed)) = xtrct_instructionVar206::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var206(parsed)));
        }
        if let Some((inst_len, parsed)) = add_instructionVar207::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var207(parsed)));
        }
        if let Some((inst_len, parsed)) = bra_instructionVar208::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var208(parsed)));
        }
        if let Some((inst_len, parsed)) = bsr_instructionVar209::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var209(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_instructionVar210::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var210(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_w_instructionVar211::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var211(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar212::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var212(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar213::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var213(parsed)));
        }
        if let Some((inst_len, parsed)) = mov_l_instructionVar214::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var214(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:460:1, end:460:5))"]
#[derive(Clone, Debug)]
struct M_0tVar0 {
    M_0: TokenField_M_0,
}
impl M_0tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.M_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let M_0 = token_parser.TokenFieldM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { M_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableM_0t {
    Var0(M_0tVar0),
}
impl TableM_0t {
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
            M_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:462:1, end:462:5))"]
#[derive(Clone, Debug)]
struct N_0tVar0 {
    N_0: TokenField_N_0,
}
impl N_0tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.N_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t {
    Var0(N_0tVar0),
}
impl TableN_0t {
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
            N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:464:1, end:464:8))"]
#[derive(Clone, Debug)]
struct N_0tjmpVar0 {
    N_0: TokenField_N_0,
}
impl N_0tjmpVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("@"), self.N_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0tjmp {
    Var0(N_0tjmpVar0),
}
impl TableN_0tjmp {
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
            N_0tjmpVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:466:1, end:466:5))"]
#[derive(Clone, Debug)]
struct I_0tVar0 {
    I_0: TokenField_I_0,
}
impl I_0tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.I_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let I_0 = token_parser.TokenFieldI_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { I_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableI_0t {
    Var0(I_0tVar0),
}
impl TableI_0t {
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
            I_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:468:1, end:468:5))"]
#[derive(Clone, Debug)]
struct U_0tVar0 {
    U_0: TokenField_U_0,
}
impl U_0tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.U_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t {
    Var0(U_0tVar0),
}
impl TableU_0t {
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
            U_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:469:1, end:469:6))"]
#[derive(Clone, Debug)]
struct U_0t1Var0 {
    U_0: TokenField_U_0,
}
impl U_0t1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("#"),
            self.U_0.display(),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
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
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t1 {
    Var0(U_0t1Var0),
}
impl TableU_0t1 {
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
            U_0t1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:471:1, end:471:8))"]
#[derive(Clone, Debug)]
struct I_0t_r0Var0 {
    I_0: TokenField_I_0,
}
impl I_0t_r0Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("#"),
            self.I_0.display(),
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
        let I_0 = token_parser.TokenFieldI_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { I_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableI_0t_r0 {
    Var0(I_0t_r0Var0),
}
impl TableI_0t_r0 {
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
            I_0t_r0Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:473:1, end:473:8))"]
#[derive(Clone, Debug)]
struct U_0t_r0Var0 {
    U_0: TokenField_U_0,
}
impl U_0t_r0Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("#"),
            self.U_0.display(),
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
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t_r0 {
    Var0(U_0t_r0Var0),
}
impl TableU_0t_r0 {
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
            U_0t_r0Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:475:1, end:475:11))"]
#[derive(Clone, Debug)]
struct I_0tbranchVar0 {
    I_0: TokenField_I_0,
}
impl I_0tbranchVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_dest: i128 = 0;
        calc_dest = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(self.I_0.disassembly().wrapping_mul(2i128))
            .wrapping_add(4i128);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_dest)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_dest: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_dest = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(
                token_parser
                    .TokenFieldI_0()
                    .disassembly()
                    .wrapping_mul(2i128),
            )
            .wrapping_add(4i128);
        let I_0 = token_parser.TokenFieldI_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { I_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableI_0tbranch {
    Var0(I_0tbranchVar0),
}
impl TableI_0tbranch {
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
        if let Some((inst_len, parsed)) = I_0tbranchVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:477:1, end:477:11))"]
#[derive(Clone, Debug)]
struct I_1tbranchVar0 {
    I_1: TokenField_I_1,
}
impl I_1tbranchVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_dest: i128 = 0;
        calc_dest = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(self.I_1.disassembly().wrapping_mul(2i128))
            .wrapping_add(4i128);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_dest)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_dest: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_dest = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(
                token_parser
                    .TokenFieldI_1()
                    .disassembly()
                    .wrapping_mul(2i128),
            )
            .wrapping_add(4i128);
        let I_1 = token_parser.TokenFieldI_1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { I_1 }))
    }
}
#[derive(Clone, Debug)]
enum TableI_1tbranch {
    Var0(I_1tbranchVar0),
}
impl TableI_1tbranch {
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
        if let Some((inst_len, parsed)) = I_1tbranchVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:479:1, end:479:8))"]
#[derive(Clone, Debug)]
struct sr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl sr_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::SR),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablesr_N_0t {
    Var0(sr_N_0tVar0),
}
impl Tablesr_N_0t {
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
            sr_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:481:1, end:481:9))"]
#[derive(Clone, Debug)]
struct gbr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl gbr_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::GBR),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablegbr_N_0t {
    Var0(gbr_N_0tVar0),
}
impl Tablegbr_N_0t {
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
            gbr_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:483:1, end:483:9))"]
#[derive(Clone, Debug)]
struct vbr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl vbr_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::VBR),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablevbr_N_0t {
    Var0(vbr_N_0tVar0),
}
impl Tablevbr_N_0t {
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
            vbr_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:485:1, end:485:9))"]
#[derive(Clone, Debug)]
struct ssr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl ssr_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::SSR),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablessr_N_0t {
    Var0(ssr_N_0tVar0),
}
impl Tablessr_N_0t {
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
            ssr_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:487:1, end:487:9))"]
#[derive(Clone, Debug)]
struct spc_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl spc_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::SPC),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablespc_N_0t {
    Var0(spc_N_0tVar0),
}
impl Tablespc_N_0t {
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
            spc_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:489:1, end:489:9))"]
#[derive(Clone, Debug)]
struct sgr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl sgr_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::SGR),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablesgr_N_0t {
    Var0(sgr_N_0tVar0),
}
impl Tablesgr_N_0t {
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
            sgr_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:491:1, end:491:9))"]
#[derive(Clone, Debug)]
struct dbr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl dbr_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::DBR),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tabledbr_N_0t {
    Var0(dbr_N_0tVar0),
}
impl Tabledbr_N_0t {
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
            dbr_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:493:1, end:493:5))"]
#[derive(Clone, Debug)]
struct sr_tVar0 {}
impl sr_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::SR)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablesr_t {
    Var0(sr_tVar0),
}
impl Tablesr_t {
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
            sr_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:495:1, end:495:6))"]
#[derive(Clone, Debug)]
struct gbr_tVar0 {}
impl gbr_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::GBR)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablegbr_t {
    Var0(gbr_tVar0),
}
impl Tablegbr_t {
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
            gbr_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:497:1, end:497:6))"]
#[derive(Clone, Debug)]
struct vbr_tVar0 {}
impl vbr_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::VBR)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablevbr_t {
    Var0(vbr_tVar0),
}
impl Tablevbr_t {
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
            vbr_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:499:1, end:499:6))"]
#[derive(Clone, Debug)]
struct ssr_tVar0 {}
impl ssr_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::SSR)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablessr_t {
    Var0(ssr_tVar0),
}
impl Tablessr_t {
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
            ssr_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:501:1, end:501:6))"]
#[derive(Clone, Debug)]
struct spc_tVar0 {}
impl spc_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::SPC)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablespc_t {
    Var0(spc_tVar0),
}
impl Tablespc_t {
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
            spc_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:503:1, end:503:6))"]
#[derive(Clone, Debug)]
struct sgr_tVar0 {}
impl sgr_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::SGR)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablesgr_t {
    Var0(sgr_tVar0),
}
impl Tablesgr_t {
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
            sgr_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:505:1, end:505:6))"]
#[derive(Clone, Debug)]
struct dbr_tVar0 {}
impl dbr_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::DBR)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tabledbr_t {
    Var0(dbr_tVar0),
}
impl Tabledbr_t {
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
            dbr_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:507:1, end:507:8))"]
#[derive(Clone, Debug)]
struct N_0t_srVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_srVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::SR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_sr {
    Var0(N_0t_srVar0),
}
impl TableN_0t_sr {
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
            N_0t_srVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:509:1, end:509:9))"]
#[derive(Clone, Debug)]
struct N_0t_gbrVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_gbrVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_gbr {
    Var0(N_0t_gbrVar0),
}
impl TableN_0t_gbr {
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
            N_0t_gbrVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:511:1, end:511:9))"]
#[derive(Clone, Debug)]
struct N_0t_vbrVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_vbrVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::VBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_vbr {
    Var0(N_0t_vbrVar0),
}
impl TableN_0t_vbr {
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
            N_0t_vbrVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:513:1, end:513:9))"]
#[derive(Clone, Debug)]
struct N_0t_ssrVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_ssrVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::SSR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_ssr {
    Var0(N_0t_ssrVar0),
}
impl TableN_0t_ssr {
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
            N_0t_ssrVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:515:1, end:515:9))"]
#[derive(Clone, Debug)]
struct N_0t_spcVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_spcVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::SPC),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_spc {
    Var0(N_0t_spcVar0),
}
impl TableN_0t_spc {
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
            N_0t_spcVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:518:1, end:518:9))"]
#[derive(Clone, Debug)]
struct N_0t_dbrVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_dbrVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::DBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_dbr {
    Var0(N_0t_dbrVar0),
}
impl TableN_0t_dbr {
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
            N_0t_dbrVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:520:1, end:520:10))"]
#[derive(Clone, Debug)]
struct N_0t_bankVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_bankVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.N_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_bank {
    Var0(N_0t_bankVar0),
}
impl TableN_0t_bank {
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
            N_0t_bankVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:522:1, end:522:9))"]
#[derive(Clone, Debug)]
struct N_0t_sr1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_sr1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::SR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_sr1 {
    Var0(N_0t_sr1Var0),
}
impl TableN_0t_sr1 {
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
            N_0t_sr1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:524:1, end:524:10))"]
#[derive(Clone, Debug)]
struct N_0t_gbr1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_gbr1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::GBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_gbr1 {
    Var0(N_0t_gbr1Var0),
}
impl TableN_0t_gbr1 {
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
            N_0t_gbr1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:526:1, end:526:10))"]
#[derive(Clone, Debug)]
struct N_0t_vbr1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_vbr1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::VBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_vbr1 {
    Var0(N_0t_vbr1Var0),
}
impl TableN_0t_vbr1 {
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
            N_0t_vbr1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:528:1, end:528:10))"]
#[derive(Clone, Debug)]
struct N_0t_ssr1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_ssr1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::SSR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_ssr1 {
    Var0(N_0t_ssr1Var0),
}
impl TableN_0t_ssr1 {
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
            N_0t_ssr1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:530:1, end:530:10))"]
#[derive(Clone, Debug)]
struct N_0t_spc1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_spc1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::SPC),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_spc1 {
    Var0(N_0t_spc1Var0),
}
impl TableN_0t_spc1 {
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
            N_0t_spc1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:533:1, end:533:10))"]
#[derive(Clone, Debug)]
struct N_0t_dbr1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_dbr1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::DBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_dbr1 {
    Var0(N_0t_dbr1Var0),
}
impl TableN_0t_dbr1 {
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
            N_0t_dbr1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:535:1, end:535:11))"]
#[derive(Clone, Debug)]
struct N_0t_bank1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_bank1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
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
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_bank1 {
    Var0(N_0t_bank1Var0),
}
impl TableN_0t_bank1 {
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
        if let Some((inst_len, parsed)) = N_0t_bank1Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:537:1, end:537:6))"]
#[derive(Clone, Debug)]
struct FR0_tVar0 {}
impl FR0_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::fr0)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableFR0_t {
    Var0(FR0_tVar0),
}
impl TableFR0_t {
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
            FR0_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:539:1, end:539:8))"]
#[derive(Clone, Debug)]
struct XMTRX_tVar0 {}
impl XMTRX_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("xmtrx")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableXMTRX_t {
    Var0(XMTRX_tVar0),
}
impl TableXMTRX_t {
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
            XMTRX_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:541:1, end:541:7))"]
#[derive(Clone, Debug)]
struct mach_tVar0 {}
impl mach_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::MACH)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablemach_t {
    Var0(mach_tVar0),
}
impl Tablemach_t {
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
            mach_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:543:1, end:543:7))"]
#[derive(Clone, Debug)]
struct macl_tVar0 {}
impl macl_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::MACL)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablemacl_t {
    Var0(macl_tVar0),
}
impl Tablemacl_t {
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
            macl_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:545:1, end:545:7))"]
#[derive(Clone, Debug)]
struct fpul_tVar0 {}
impl fpul_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Register(Register::FPUL)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablefpul_t {
    Var0(fpul_tVar0),
}
impl Tablefpul_t {
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
            fpul_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:547:1, end:547:8))"]
#[derive(Clone, Debug)]
struct fpscr_tVar0 {}
impl fpscr_tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("FPSCR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let OP_0 = token_parser.TokenFieldOP_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablefpscr_t {
    Var0(fpscr_tVar0),
}
impl Tablefpscr_t {
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
            fpscr_tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:549:1, end:549:10))"]
#[derive(Clone, Debug)]
struct mach_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl mach_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::MACH),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablemach_N_0t {
    Var0(mach_N_0tVar0),
}
impl Tablemach_N_0t {
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
            mach_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:551:1, end:551:10))"]
#[derive(Clone, Debug)]
struct macl_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl macl_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::MACL),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablemacl_N_0t {
    Var0(macl_N_0tVar0),
}
impl Tablemacl_N_0t {
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
            macl_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:553:1, end:553:8))"]
#[derive(Clone, Debug)]
struct pr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl pr_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::PR),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablepr_N_0t {
    Var0(pr_N_0tVar0),
}
impl Tablepr_N_0t {
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
            pr_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:555:1, end:555:10))"]
#[derive(Clone, Debug)]
struct fpul_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl fpul_N_0tVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablefpul_N_0t {
    Var0(fpul_N_0tVar0),
}
impl Tablefpul_N_0t {
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
            fpul_N_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:557:1, end:557:11))"]
#[derive(Clone, Debug)]
struct fpscr_N_0tVar0 {
    N_0: TokenField_N_0,
}
impl fpscr_N_0tVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("FPSCR,"), self.N_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum Tablefpscr_N_0t {
    Var0(fpscr_N_0tVar0),
}
impl Tablefpscr_N_0t {
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
        if let Some((inst_len, parsed)) = fpscr_N_0tVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:559:1, end:559:10))"]
#[derive(Clone, Debug)]
struct N_0t_machVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_machVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::MACH),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_mach {
    Var0(N_0t_machVar0),
}
impl TableN_0t_mach {
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
            N_0t_machVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:561:1, end:561:10))"]
#[derive(Clone, Debug)]
struct N_0t_maclVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_maclVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::MACL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_macl {
    Var0(N_0t_maclVar0),
}
impl TableN_0t_macl {
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
            N_0t_maclVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:563:1, end:563:8))"]
#[derive(Clone, Debug)]
struct N_0t_prVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_prVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::PR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_pr {
    Var0(N_0t_prVar0),
}
impl TableN_0t_pr {
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
            N_0t_prVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:565:1, end:565:10))"]
#[derive(Clone, Debug)]
struct N_0t_fpulVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_fpulVar0 {
    fn display_extend<T>(
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
            self.N_0.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_fpul {
    Var0(N_0t_fpulVar0),
}
impl TableN_0t_fpul {
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
            N_0t_fpulVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:567:1, end:567:11))"]
#[derive(Clone, Debug)]
struct N_0t_fpscrVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_fpscrVar0 {
    fn display_extend<T>(
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
            [self.N_0.display(), DisplayElement::Literal(",fpscr")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_fpscr {
    Var0(N_0t_fpscrVar0),
}
impl TableN_0t_fpscr {
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
        if let Some((inst_len, parsed)) = N_0t_fpscrVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:569:1, end:569:11))"]
#[derive(Clone, Debug)]
struct N_0t_mach1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_mach1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::MACH),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_mach1 {
    Var0(N_0t_mach1Var0),
}
impl TableN_0t_mach1 {
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
        if let Some((inst_len, parsed)) = N_0t_mach1Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:571:1, end:571:11))"]
#[derive(Clone, Debug)]
struct N_0t_macl1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_macl1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::MACL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_macl1 {
    Var0(N_0t_macl1Var0),
}
impl TableN_0t_macl1 {
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
        if let Some((inst_len, parsed)) = N_0t_macl1Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:573:1, end:573:9))"]
#[derive(Clone, Debug)]
struct N_0t_pr1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_pr1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::PR),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_pr1 {
    Var0(N_0t_pr1Var0),
}
impl TableN_0t_pr1 {
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
            N_0t_pr1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:575:1, end:575:11))"]
#[derive(Clone, Debug)]
struct N_0t_fpul1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_fpul1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_fpul1 {
    Var0(N_0t_fpul1Var0),
}
impl TableN_0t_fpul1 {
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
        if let Some((inst_len, parsed)) = N_0t_fpul1Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:577:1, end:577:12))"]
#[derive(Clone, Debug)]
struct N_0t_fpscr1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_fpscr1Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
            DisplayElement::Literal("+,fpscr"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_fpscr1 {
    Var0(N_0t_fpscr1Var0),
}
impl TableN_0t_fpscr1 {
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
        if let Some((inst_len, parsed)) = N_0t_fpscr1Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:579:1, end:579:9))"]
#[derive(Clone, Debug)]
struct M_0t_at1Var0 {
    M_0: TokenField_M_0,
}
impl M_0t_at1Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("@"), self.M_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let M_0 = token_parser.TokenFieldM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { M_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableM_0t_at1 {
    Var0(M_0t_at1Var0),
}
impl TableM_0t_at1 {
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
            M_0t_at1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:581:1, end:581:9))"]
#[derive(Clone, Debug)]
struct N_0t_at1Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_at1Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("@"), self.N_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_at1 {
    Var0(N_0t_at1Var0),
}
impl TableN_0t_at1 {
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
            N_0t_at1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:583:1, end:583:8))"]
#[derive(Clone, Debug)]
struct M_0t_atVar0 {
    M_0: TokenField_M_0,
}
impl M_0t_atVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.M_0.display(),
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
        let M_0 = token_parser.TokenFieldM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { M_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableM_0t_at {
    Var0(M_0t_atVar0),
}
impl TableM_0t_at {
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
            M_0t_atVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:585:1, end:585:8))"]
#[derive(Clone, Debug)]
struct N_0t_atVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_atVar0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@"),
            self.N_0.display(),
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
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_at {
    Var0(N_0t_atVar0),
}
impl TableN_0t_at {
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
            N_0t_atVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:587:1, end:587:7))"]
#[derive(Clone, Debug)]
struct FRM_0tVar0 {
    FRM_0: TokenField_FRM_0,
}
impl FRM_0tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.FRM_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let FRM_0 = token_parser.TokenFieldFRM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRM_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableFRM_0t {
    Var0(FRM_0tVar0),
}
impl TableFRM_0t {
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
            FRM_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:589:1, end:589:7))"]
#[derive(Clone, Debug)]
struct FRN_0tVar0 {
    FRN_0: TokenField_FRN_0,
}
impl FRN_0tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.FRN_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let FRN_0 = token_parser.TokenFieldFRN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FRN_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableFRN_0t {
    Var0(FRN_0tVar0),
}
impl TableFRN_0t {
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
            FRN_0tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:591:1, end:591:7))"]
#[derive(Clone, Debug)]
struct DRM_1tVar0 {
    DRM_1: TokenField_DRM_1,
}
impl DRM_1tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.DRM_1.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let DRM_1 = token_parser.TokenFieldDRM_1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DRM_1 }))
    }
}
#[derive(Clone, Debug)]
enum TableDRM_1t {
    Var0(DRM_1tVar0),
}
impl TableDRM_1t {
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
            DRM_1tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:593:1, end:593:7))"]
#[derive(Clone, Debug)]
struct DRN_1tVar0 {
    DRN_1: TokenField_DRN_1,
}
impl DRN_1tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.DRN_1.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let DRN_1 = token_parser.TokenFieldDRN_1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DRN_1 }))
    }
}
#[derive(Clone, Debug)]
enum TableDRN_1t {
    Var0(DRN_1tVar0),
}
impl TableDRN_1t {
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
            DRN_1tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:595:1, end:595:7))"]
#[derive(Clone, Debug)]
struct FVM_2tVar0 {
    FVM_2: TokenField_FVM_2,
}
impl FVM_2tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.FVM_2.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let FVM_2 = token_parser.TokenFieldFVM_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FVM_2 }))
    }
}
#[derive(Clone, Debug)]
enum TableFVM_2t {
    Var0(FVM_2tVar0),
}
impl TableFVM_2t {
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
            FVM_2tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:597:1, end:597:7))"]
#[derive(Clone, Debug)]
struct FVN_2tVar0 {
    FVN_2: TokenField_FVN_2,
}
impl FVN_2tVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.FVN_2.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let FVN_2 = token_parser.TokenFieldFVN_2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { FVN_2 }))
    }
}
#[derive(Clone, Debug)]
enum TableFVN_2t {
    Var0(FVN_2tVar0),
}
impl TableFVN_2t {
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
            FVN_2tVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:599:1, end:599:16))"]
#[derive(Clone, Debug)]
struct N_0t_at_with_r0Var0 {
    N_0: TokenField_N_0,
}
impl N_0t_at_with_r0Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.N_0.display(),
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
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_at_with_r0 {
    Var0(N_0t_at_with_r0Var0),
}
impl TableN_0t_at_with_r0 {
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
        if let Some((inst_len, parsed)) = N_0t_at_with_r0Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:601:1, end:601:16))"]
#[derive(Clone, Debug)]
struct M_0t_at_with_r0Var0 {
    M_0: TokenField_M_0,
}
impl M_0t_at_with_r0Var0 {
    fn display_extend<T>(
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
            DisplayElement::Literal("@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            self.M_0.display(),
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
        let M_0 = token_parser.TokenFieldM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { M_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableM_0t_at_with_r0 {
    Var0(M_0t_at_with_r0Var0),
}
impl TableM_0t_at_with_r0 {
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
        if let Some((inst_len, parsed)) = M_0t_at_with_r0Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:603:1, end:603:12))"]
#[derive(Clone, Debug)]
struct N_0t_at_negVar0 {
    N_0: TokenField_N_0,
}
impl N_0t_at_negVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("@-"), self.N_0.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0t_at_neg {
    Var0(N_0t_at_negVar0),
}
impl TableN_0t_at_neg {
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
        if let Some((inst_len, parsed)) = N_0t_at_negVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:605:1, end:605:16))"]
#[derive(Clone, Debug)]
struct U_2t_M0_dispr01Var0 {
    M_0: TokenField_M_0,
    U_2: TokenField_U_2,
}
impl U_2t_M0_dispr01Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i128 = 0;
        calc_disp = self.U_2.disassembly().wrapping_mul(1i128);
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.M_0.display(),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFieldU_2()
            .disassembly()
            .wrapping_mul(1i128);
        let U_2 = token_parser.TokenFieldU_2();
        let M_0 = token_parser.TokenFieldM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_2, M_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_2t_M0_dispr01 {
    Var0(U_2t_M0_dispr01Var0),
}
impl TableU_2t_M0_dispr01 {
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
        if let Some((inst_len, parsed)) = U_2t_M0_dispr01Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:606:1, end:606:16))"]
#[derive(Clone, Debug)]
struct U_2t_M0_dispr02Var0 {
    M_0: TokenField_M_0,
    U_2: TokenField_U_2,
}
impl U_2t_M0_dispr02Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i128 = 0;
        calc_disp = self.U_2.disassembly().wrapping_mul(2i128);
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.M_0.display(),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFieldU_2()
            .disassembly()
            .wrapping_mul(2i128);
        let U_2 = token_parser.TokenFieldU_2();
        let M_0 = token_parser.TokenFieldM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_2, M_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_2t_M0_dispr02 {
    Var0(U_2t_M0_dispr02Var0),
}
impl TableU_2t_M0_dispr02 {
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
        if let Some((inst_len, parsed)) = U_2t_M0_dispr02Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:607:1, end:607:16))"]
#[derive(Clone, Debug)]
struct U_2t_M0_dispr04Var0 {
    M_0: TokenField_M_0,
    U_2: TokenField_U_2,
}
impl U_2t_M0_dispr04Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i128 = 0;
        calc_disp = self.U_2.disassembly().wrapping_mul(4i128);
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.M_0.display(),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFieldU_2()
            .disassembly()
            .wrapping_mul(4i128);
        let U_2 = token_parser.TokenFieldU_2();
        let M_0 = token_parser.TokenFieldM_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_2, M_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_2t_M0_dispr04 {
    Var0(U_2t_M0_dispr04Var0),
}
impl TableU_2t_M0_dispr04 {
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
        if let Some((inst_len, parsed)) = U_2t_M0_dispr04Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:609:1, end:609:16))"]
#[derive(Clone, Debug)]
struct U_2t_N0_dispr04Var0 {
    N_0: TokenField_N_0,
    U_2: TokenField_U_2,
}
impl U_2t_N0_dispr04Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i128 = 0;
        calc_disp = self.U_2.disassembly().wrapping_mul(4i128);
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            self.N_0.display(),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFieldU_2()
            .disassembly()
            .wrapping_mul(4i128);
        let U_2 = token_parser.TokenFieldU_2();
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_2, N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_2t_N0_dispr04 {
    Var0(U_2t_N0_dispr04Var0),
}
impl TableU_2t_N0_dispr04 {
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
        if let Some((inst_len, parsed)) = U_2t_N0_dispr04Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:612:1, end:612:14))"]
#[derive(Clone, Debug)]
struct U_0t_gbr_at_1Var0 {
    U_0: TokenField_U_0,
}
impl U_0t_gbr_at_1Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i128 = 0;
        calc_disp = self.U_0.disassembly().wrapping_mul(1i128);
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFieldU_0()
            .disassembly()
            .wrapping_mul(1i128);
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t_gbr_at_1 {
    Var0(U_0t_gbr_at_1Var0),
}
impl TableU_0t_gbr_at_1 {
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
        if let Some((inst_len, parsed)) = U_0t_gbr_at_1Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:613:1, end:613:14))"]
#[derive(Clone, Debug)]
struct U_0t_gbr_at_2Var0 {
    U_0: TokenField_U_0,
}
impl U_0t_gbr_at_2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i128 = 0;
        calc_disp = self.U_0.disassembly().wrapping_mul(2i128);
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFieldU_0()
            .disassembly()
            .wrapping_mul(2i128);
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t_gbr_at_2 {
    Var0(U_0t_gbr_at_2Var0),
}
impl TableU_0t_gbr_at_2 {
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
        if let Some((inst_len, parsed)) = U_0t_gbr_at_2Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:614:1, end:614:14))"]
#[derive(Clone, Debug)]
struct U_0t_gbr_at_4Var0 {
    U_0: TokenField_U_0,
}
impl U_0t_gbr_at_4Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_disp: i128 = 0;
        calc_disp = self.U_0.disassembly().wrapping_mul(4i128);
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Number(true, calc_disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_disp = token_parser
            .TokenFieldU_0()
            .disassembly()
            .wrapping_mul(4i128);
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t_gbr_at_4 {
    Var0(U_0t_gbr_at_4Var0),
}
impl TableU_0t_gbr_at_4 {
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
        if let Some((inst_len, parsed)) = U_0t_gbr_at_4Var0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:621:1, end:621:9))"]
#[derive(Clone, Debug)]
struct U_0t_2pcVar0 {
    U_0: TokenField_U_0,
}
impl U_0t_2pcVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_dest: i128 = 0;
        calc_dest = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(self.U_0.disassembly().wrapping_mul(2i128))
            .wrapping_add(4i128);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_dest)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_dest: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_dest = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(
                token_parser
                    .TokenFieldU_0()
                    .disassembly()
                    .wrapping_mul(2i128),
            )
            .wrapping_add(4i128);
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t_2pc {
    Var0(U_0t_2pcVar0),
}
impl TableU_0t_2pc {
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
            U_0t_2pcVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:623:1, end:623:9))"]
#[derive(Clone, Debug)]
struct U_0t_4pcVar0 {
    U_0: TokenField_U_0,
}
impl U_0t_4pcVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_dest: i128 = 0;
        calc_dest = (i128::try_from(inst_start).unwrap() & 4294967292i128)
            .wrapping_add(self.U_0.disassembly().wrapping_mul(4i128))
            .wrapping_add(4i128);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_dest)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut calc_dest: i128 = 0;
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        calc_dest = (i128::try_from(inst_start).unwrap() & 4294967292i128)
            .wrapping_add(
                token_parser
                    .TokenFieldU_0()
                    .disassembly()
                    .wrapping_mul(4i128),
            )
            .wrapping_add(4i128);
        let U_0 = token_parser.TokenFieldU_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { U_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableU_0t_4pc {
    Var0(U_0t_4pcVar0),
}
impl TableU_0t_4pc {
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
            U_0t_4pcVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:625:1, end:625:6))"]
#[derive(Clone, Debug)]
struct BANKtVar0 {
    BANK: TokenField_BANK,
}
impl BANKtVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.BANK.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let BANK = token_parser.TokenFieldBANK();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { BANK }))
    }
}
#[derive(Clone, Debug)]
enum TableBANKt {
    Var0(BANKtVar0),
}
impl TableBANKt {
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
            BANKtVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc, start:627:1, end:627:7))"]
#[derive(Clone, Debug)]
struct N_0txxVar0 {
    N_0: TokenField_N_0,
}
impl N_0txxVar0 {
    fn display_extend<T>(
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
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(",@"),
            self.N_0.display(),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let N_0 = token_parser.TokenFieldN_0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { N_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableN_0txx {
    Var0(N_0txxVar0),
}
impl TableN_0txx {
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
            N_0txxVar0::parse(tokens_param, &mut context_current, inst_start)
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
