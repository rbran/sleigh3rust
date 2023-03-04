use sleigh4rust::*;
pub type AddrType = u16;
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
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
    fn disassembly(&self) -> i128 {
        i128::try_from(self.0).unwrap()
    }
    fn display(&self) -> DisplayElement {
        meaning_number(true, self.0)
    }
}
#[derive(Clone, Copy, Debug)]
struct TokenField_adata(u8);
impl TokenField_adata {
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
    fn TokenFieldopfull(&self) -> TokenField_opfull {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_opfull(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldoplo(&self) -> TokenField_oplo {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_oplo(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldophi(&self) -> TokenField_ophi {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_ophi(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrn(&self) -> TokenField_rn {
        let inner_value = self.read_u8::<true>(0, 0, 3).unwrap();
        TokenField_rn(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrnfill(&self) -> TokenField_rnfill {
        let inner_value = self.read_u8::<true>(0, 3, 1).unwrap();
        TokenField_rnfill(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldri(&self) -> TokenField_ri {
        let inner_value = self.read_u8::<true>(0, 0, 1).unwrap();
        TokenField_ri(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrifill(&self) -> TokenField_rifill {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_rifill(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopaddr(&self) -> TokenField_opaddr {
        let inner_value = self.read_u8::<true>(0, 5, 3).unwrap();
        TokenField_opaddr(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldaddrfill(&self) -> TokenField_addrfill {
        let inner_value = self.read_u8::<true>(0, 4, 1).unwrap();
        TokenField_addrfill(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0000(&self) -> TokenField_b_0000 {
        let inner_value = self.read_u8::<true>(0, 0, 1).unwrap();
        TokenField_b_0000(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0001(&self) -> TokenField_b_0001 {
        let inner_value = self.read_u8::<true>(0, 0, 2).unwrap();
        TokenField_b_0001(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0002(&self) -> TokenField_b_0002 {
        let inner_value = self.read_u8::<true>(0, 0, 3).unwrap();
        TokenField_b_0002(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0005(&self) -> TokenField_b_0005 {
        let inner_value = self.read_u8::<true>(0, 0, 6).unwrap();
        TokenField_b_0005(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0101(&self) -> TokenField_b_0101 {
        let inner_value = self.read_u8::<true>(0, 1, 1).unwrap();
        TokenField_b_0101(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0107(&self) -> TokenField_b_0107 {
        let inner_value = self.read_u8::<true>(0, 1, 7).unwrap();
        TokenField_b_0107(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0207(&self) -> TokenField_b_0207 {
        let inner_value = self.read_u8::<true>(0, 2, 6).unwrap();
        TokenField_b_0207(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0307(&self) -> TokenField_b_0307 {
        let inner_value = self.read_u8::<true>(0, 3, 5).unwrap();
        TokenField_b_0307(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb_0607(&self) -> TokenField_b_0607 {
        let inner_value = self.read_u8::<true>(0, 6, 2).unwrap();
        TokenField_b_0607(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddirect(&self) -> TokenField_direct {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_direct(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbank(&self) -> TokenField_bank {
        let inner_value = self.read_u8::<true>(0, 7, 1).unwrap();
        TokenField_bank(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfr(&self) -> TokenField_sfr {
        let inner_value = self.read_u8::<true>(0, 0, 7).unwrap();
        TokenField_sfr(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfr6(&self) -> TokenField_sfr6 {
        let inner_value = self.read_u8::<true>(0, 6, 1).unwrap();
        TokenField_sfr6(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfrlo(&self) -> TokenField_sfrlo {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_sfrlo(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmainreg(&self) -> TokenField_mainreg {
        let inner_value = self.read_u8::<true>(0, 0, 7).unwrap();
        TokenField_mainreg(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddirect17(&self) -> TokenField_direct17 {
        let inner_value = self.read_u8::<true>(0, 1, 7).unwrap();
        TokenField_direct17(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddirect2(&self) -> TokenField_direct2 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_direct2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbank2(&self) -> TokenField_bank2 {
        let inner_value = self.read_u8::<true>(0, 7, 1).unwrap();
        TokenField_bank2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfr2(&self) -> TokenField_sfr2 {
        let inner_value = self.read_u8::<true>(0, 0, 7).unwrap();
        TokenField_sfr2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfr26(&self) -> TokenField_sfr26 {
        let inner_value = self.read_u8::<true>(0, 6, 1).unwrap();
        TokenField_sfr26(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfr2lo(&self) -> TokenField_sfr2lo {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_sfr2lo(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldmainreg2(&self) -> TokenField_mainreg2 {
        let inner_value = self.read_u8::<true>(0, 0, 7).unwrap();
        TokenField_mainreg2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbitaddr8(&self) -> TokenField_bitaddr8 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_bitaddr8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbitaddr27(&self) -> TokenField_bitaddr27 {
        let inner_value = self.read_u8::<true>(0, 2, 6).unwrap();
        TokenField_bitaddr27(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbitbank(&self) -> TokenField_bitbank {
        let inner_value = self.read_u8::<true>(0, 7, 1).unwrap();
        TokenField_bitbank(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfrbyte(&self) -> TokenField_sfrbyte {
        let inner_value = self.read_u8::<true>(0, 3, 5).unwrap();
        TokenField_sfrbyte(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbitaddr57(&self) -> TokenField_bitaddr57 {
        let inner_value = self.read_u8::<true>(0, 5, 3).unwrap();
        TokenField_bitaddr57(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfrbit6(&self) -> TokenField_sfrbit6 {
        let inner_value = self.read_u8::<true>(0, 6, 1).unwrap();
        TokenField_sfrbit6(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfrbit3(&self) -> TokenField_sfrbit3 {
        let inner_value = self.read_u8::<true>(0, 3, 1).unwrap();
        TokenField_sfrbit3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsfrbit(&self) -> TokenField_sfrbit {
        let inner_value = self.read_u8::<true>(0, 0, 3).unwrap();
        TokenField_sfrbit(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldlowbyte(&self) -> TokenField_lowbyte {
        let inner_value = self.read_u8::<true>(0, 3, 4).unwrap();
        TokenField_lowbyte(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbitaddr0(&self) -> TokenField_bitaddr0 {
        let inner_value = self.read_u8::<true>(0, 0, 1).unwrap();
        TokenField_bitaddr0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldaddr16(&self) -> TokenField_addr16 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_addr16(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldrel8(&self) -> TokenField_rel8 {
        let inner_value = self.read_i8::<true>(0, 0, 8).unwrap();
        TokenField_rel8(i8::try_from(inner_value).unwrap())
    }
    fn TokenFielddata(&self) -> TokenField_data {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_data(u8::try_from(inner_value).unwrap())
    }
    fn TokenFielddata16(&self) -> TokenField_data16 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_data16(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldrel16(&self) -> TokenField_rel16 {
        let inner_value = self.read_i16::<true>(0, 0, 16).unwrap();
        TokenField_rel16(i16::try_from(inner_value).unwrap())
    }
    fn TokenFieldaoplo(&self) -> TokenField_aoplo {
        let inner_value = self.read_u8::<true>(0, 0, 4).unwrap();
        TokenField_aoplo(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldaopaddr(&self) -> TokenField_aopaddr {
        let inner_value = self.read_u8::<true>(0, 5, 3).unwrap();
        TokenField_aopaddr(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldaaddrfill(&self) -> TokenField_aaddrfill {
        let inner_value = self.read_u8::<true>(0, 4, 1).unwrap();
        TokenField_aaddrfill(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldadata(&self) -> TokenField_adata {
        let inner_value = self.read_u8::<true>(1, 0, 8).unwrap();
        TokenField_adata(u8::try_from(inner_value).unwrap())
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:652:1, end:652:2))"]
#[derive(Clone, Debug)]
struct ACALL_instructionVar0 {
    Addr11: TableAddr11,
}
impl ACALL_instructionVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ACALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaaddrfill().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldaoplo().disassembly() != 1i128 {
            return None;
        }
        let Addr11 = if let Some((len, table)) = TableAddr11::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:679:1, end:679:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar1 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl ANL_instructionVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:680:1, end:680:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar2 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
}
impl ANL_instructionVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:700:1, end:700:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar3 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl CLR_instructionVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:713:1, end:713:2))"]
#[derive(Clone, Debug)]
struct CPL_instructionVar4 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl CPL_instructionVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:749:1, end:749:2))"]
#[derive(Clone, Debug)]
struct JB_instructionVar5 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
}
impl JB_instructionVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JB"), DisplayElement::Literal(" ")];
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:750:1, end:750:2))"]
#[derive(Clone, Debug)]
struct JBC_instructionVar6 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
}
impl JBC_instructionVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:764:1, end:764:2))"]
#[derive(Clone, Debug)]
struct JNB_instructionVar7 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
}
impl JNB_instructionVar7 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:809:1, end:809:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar8 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl MOV_instructionVar8 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:810:1, end:810:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar9 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl MOV_instructionVar9 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:842:1, end:842:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar10 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl ORL_instructionVar10 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:843:1, end:843:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar11 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
}
impl ORL_instructionVar11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:886:1, end:886:2))"]
#[derive(Clone, Debug)]
struct SETB_instructionVar12 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl SETB_instructionVar12 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitaddr57().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbit3().disassembly() != 0i128 {
            return None;
        }
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:656:1, end:656:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar13 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl ADD_instructionVar13 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:658:1, end:658:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar14 {
    Areg: TableAreg,
    Data: TableData,
}
impl ADD_instructionVar14 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:661:1, end:661:2))"]
#[derive(Clone, Debug)]
struct ADDC_instructionVar15 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl ADDC_instructionVar15 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:663:1, end:663:2))"]
#[derive(Clone, Debug)]
struct ADDC_instructionVar16 {
    Areg: TableAreg,
    Data: TableData,
}
impl ADDC_instructionVar16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:673:1, end:673:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar17 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl ANL_instructionVar17 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:675:1, end:675:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar18 {
    Areg: TableAreg,
    Data: TableData,
}
impl ANL_instructionVar18 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:676:1, end:676:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar19 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl ANL_instructionVar19 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:677:1, end:677:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar20 {
    Direct: TableDirect,
    Data: TableData,
}
impl ANL_instructionVar20 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:682:1, end:682:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar21 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl ANL_instructionVar21 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:683:1, end:683:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar22 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
}
impl ANL_instructionVar22 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:692:1, end:692:2))"]
#[derive(Clone, Debug)]
struct CJNE_instructionVar23 {
    Areg: TableAreg,
    Direct: TableDirect,
    Rel8: TableRel8,
}
impl CJNE_instructionVar23 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:693:1, end:693:2))"]
#[derive(Clone, Debug)]
struct CJNE_instructionVar24 {
    Areg: TableAreg,
    Data: TableData,
    Rel8: TableRel8,
}
impl CJNE_instructionVar24 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:697:1, end:697:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar25 {
    Areg: TableAreg,
}
impl CLR_instructionVar25 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:698:1, end:698:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar26 {
    CY: TableCY,
}
impl CLR_instructionVar26 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:702:1, end:702:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar27 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl CLR_instructionVar27 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:710:1, end:710:2))"]
#[derive(Clone, Debug)]
struct CPL_instructionVar28 {
    Areg: TableAreg,
}
impl CPL_instructionVar28 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:711:1, end:711:2))"]
#[derive(Clone, Debug)]
struct CPL_instructionVar29 {
    CY: TableCY,
}
impl CPL_instructionVar29 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:715:1, end:715:2))"]
#[derive(Clone, Debug)]
struct CPL_instructionVar30 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl CPL_instructionVar30 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:722:1, end:722:2))"]
#[derive(Clone, Debug)]
struct DA_instructionVar31 {
    Areg: TableAreg,
}
impl DA_instructionVar31 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:724:1, end:724:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar32 {
    Areg: TableAreg,
}
impl DEC_instructionVar32 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:726:1, end:726:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar33 {
    Direct: TableDirect,
}
impl DEC_instructionVar33 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:729:1, end:729:2))"]
#[derive(Clone, Debug)]
struct DIV_instructionVar34 {
    ABreg: TableABreg,
}
impl DIV_instructionVar34 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let ABreg = if let Some((len, table)) =
            TableABreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:733:1, end:733:2))"]
#[derive(Clone, Debug)]
struct DJNZ_instructionVar35 {
    Direct: TableDirect,
    Rel8: TableRel8,
}
impl DJNZ_instructionVar35 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:735:1, end:735:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar36 {
    Areg: TableAreg,
}
impl INC_instructionVar36 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:737:1, end:737:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar37 {
    Direct: TableDirect,
}
impl INC_instructionVar37 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:747:1, end:747:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar38 {
    DPTRreg: TableDPTRreg,
}
impl INC_instructionVar38 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let DPTRreg = if let Some((len, table)) = TableDPTRreg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:752:1, end:752:2))"]
#[derive(Clone, Debug)]
struct JB_instructionVar39 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
}
impl JB_instructionVar39 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JB"), DisplayElement::Literal(" ")];
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:753:1, end:753:2))"]
#[derive(Clone, Debug)]
struct JBC_instructionVar40 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
}
impl JBC_instructionVar40 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:761:1, end:761:2))"]
#[derive(Clone, Debug)]
struct JC_instructionVar41 {
    Rel8: TableRel8,
}
impl JC_instructionVar41 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("JC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:762:1, end:762:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar42 {
    ADPTR: TableADPTR,
}
impl JMP_instructionVar42 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let ADPTR = if let Some((len, table)) =
            TableADPTR::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:766:1, end:766:2))"]
#[derive(Clone, Debug)]
struct JNB_instructionVar43 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
    Rel8: TableRel8,
}
impl JNB_instructionVar43 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let sfrbit = token_parser.TokenFieldsfrbit();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:773:1, end:773:2))"]
#[derive(Clone, Debug)]
struct JNC_instructionVar44 {
    Rel8: TableRel8,
}
impl JNC_instructionVar44 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:774:1, end:774:2))"]
#[derive(Clone, Debug)]
struct JNZ_instructionVar45 {
    Rel8: TableRel8,
}
impl JNZ_instructionVar45 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:775:1, end:775:2))"]
#[derive(Clone, Debug)]
struct JZ_instructionVar46 {
    Rel8: TableRel8,
}
impl JZ_instructionVar46 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:784:1, end:784:2))"]
#[derive(Clone, Debug)]
struct LCALL_instructionVar47 {
    Addr16: TableAddr16,
}
impl LCALL_instructionVar47 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LCALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Addr16 = if let Some((len, table)) = TableAddr16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:785:1, end:785:2))"]
#[derive(Clone, Debug)]
struct LJMP_instructionVar48 {
    Addr16: TableAddr16,
}
impl LJMP_instructionVar48 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("LJMP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Addr16 = if let Some((len, table)) = TableAddr16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:789:1, end:789:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar49 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl MOV_instructionVar49 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:791:1, end:791:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar50 {
    Areg: TableAreg,
    Data: TableData,
}
impl MOV_instructionVar50 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:795:1, end:795:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar51 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl MOV_instructionVar51 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:797:1, end:797:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar52 {
    Direct: TableDirect,
    Direct2: TableDirect2,
}
impl MOV_instructionVar52 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Direct2 = if let Some((len, table)) = TableDirect2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:799:1, end:799:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar53 {
    Direct: TableDirect,
    Data: TableData,
}
impl MOV_instructionVar53 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:806:1, end:806:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar54 {
    DPTRreg: TableDPTRreg,
    Data16: TableData16,
}
impl MOV_instructionVar54 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        self.Data16.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        let DPTRreg = if let Some((len, table)) = TableDPTRreg::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data16 = if let Some((len, table)) = TableData16::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DPTRreg, Data16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:812:1, end:812:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar55 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl MOV_instructionVar55 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:813:1, end:813:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar56 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl MOV_instructionVar56 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:822:1, end:822:2))"]
#[derive(Clone, Debug)]
struct MOVC_instructionVar57 {
    ADPTR: TableADPTR,
    Areg: TableAreg,
}
impl MOVC_instructionVar57 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVC"),
            DisplayElement::Literal(" "),
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let ADPTR = if let Some((len, table)) =
            TableADPTR::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:823:1, end:823:2))"]
#[derive(Clone, Debug)]
struct MOVC_instructionVar58 {
    APC: TableAPC,
    Areg: TableAreg,
}
impl MOVC_instructionVar58 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVC"),
            DisplayElement::Literal(" "),
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let APC = if let Some((len, table)) =
            TableAPC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:826:1, end:826:2))"]
#[derive(Clone, Debug)]
struct MOVX_instructionVar59 {
    Areg: TableAreg,
    ATDPTR: TableATDPTR,
}
impl MOVX_instructionVar59 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal(" "),
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let ATDPTR = if let Some((len, table)) = TableATDPTR::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:828:1, end:828:2))"]
#[derive(Clone, Debug)]
struct MOVX_instructionVar60 {
    Areg: TableAreg,
    ATDPTR: TableATDPTR,
}
impl MOVX_instructionVar60 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal(" "),
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let ATDPTR = if let Some((len, table)) = TableATDPTR::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:830:1, end:830:2))"]
#[derive(Clone, Debug)]
struct MUL_instructionVar61 {
    ABreg: TableABreg,
}
impl MUL_instructionVar61 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let ABreg = if let Some((len, table)) =
            TableABreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:833:1, end:833:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar62 {}
impl NOP_instructionVar62 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:836:1, end:836:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar63 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl ORL_instructionVar63 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:838:1, end:838:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar64 {
    Areg: TableAreg,
    Data: TableData,
}
impl ORL_instructionVar64 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:839:1, end:839:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar65 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl ORL_instructionVar65 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:840:1, end:840:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar66 {
    Areg: TableAreg,
    Direct: TableDirect,
    Data: TableData,
}
impl ORL_instructionVar66 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:845:1, end:845:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar67 {
    CY: TableCY,
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl ORL_instructionVar67 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:846:1, end:846:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar68 {
    CY: TableCY,
    BitAddr2: TableBitAddr2,
    BitByteAddr: TableBitByteAddr,
}
impl ORL_instructionVar68 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr2 = if let Some((len, table)) = TableBitAddr2::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:855:1, end:855:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar69 {
    Direct: TableDirect,
}
impl POP_instructionVar69 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("POP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Direct.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:857:1, end:857:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar70 {
    Direct: TableDirect,
}
impl PUSH_instructionVar70 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:859:1, end:859:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar71 {}
impl RET_instructionVar71 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:869:1, end:869:2))"]
#[derive(Clone, Debug)]
struct RETI_instructionVar72 {}
impl RETI_instructionVar72 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:879:1, end:879:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar73 {
    Areg: TableAreg,
}
impl RL_instructionVar73 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("RL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:880:1, end:880:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar74 {
    Areg: TableAreg,
}
impl RLC_instructionVar74 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("RLC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:881:1, end:881:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar75 {
    Areg: TableAreg,
}
impl RR_instructionVar75 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("RR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:882:1, end:882:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar76 {
    Areg: TableAreg,
}
impl RRC_instructionVar76 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("RRC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:884:1, end:884:2))"]
#[derive(Clone, Debug)]
struct SETB_instructionVar77 {
    CY: TableCY,
}
impl SETB_instructionVar77 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SETB"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.CY.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        let CY = if let Some((len, table)) =
            TableCY::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:888:1, end:888:2))"]
#[derive(Clone, Debug)]
struct SETB_instructionVar78 {
    BitAddr: TableBitAddr,
    BitByteAddr: TableBitByteAddr,
}
impl SETB_instructionVar78 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let BitAddr = if let Some((len, table)) = TableBitAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        let BitByteAddr = if let Some((len, table)) = TableBitByteAddr::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:895:1, end:895:2))"]
#[derive(Clone, Debug)]
struct SJMP_instructionVar79 {
    Rel8: TableRel8,
}
impl SJMP_instructionVar79 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:898:1, end:898:2))"]
#[derive(Clone, Debug)]
struct SUBB_instructionVar80 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl SUBB_instructionVar80 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:900:1, end:900:2))"]
#[derive(Clone, Debug)]
struct SUBB_instructionVar81 {
    Areg: TableAreg,
    Data: TableData,
}
impl SUBB_instructionVar81 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:902:1, end:902:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar82 {
    Areg: TableAreg,
}
impl SWAP_instructionVar82 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SWAP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Areg.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:905:1, end:905:2))"]
#[derive(Clone, Debug)]
struct XCH_instructionVar83 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl XCH_instructionVar83 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XCH"), DisplayElement::Literal(" ")];
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:912:1, end:912:2))"]
#[derive(Clone, Debug)]
struct XRL_instructionVar84 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl XRL_instructionVar84 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 5i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:914:1, end:914:2))"]
#[derive(Clone, Debug)]
struct XRL_instructionVar85 {
    Areg: TableAreg,
    Data: TableData,
}
impl XRL_instructionVar85 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 4i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:915:1, end:915:2))"]
#[derive(Clone, Debug)]
struct XRL_instructionVar86 {
    Areg: TableAreg,
    Direct: TableDirect,
}
impl XRL_instructionVar86 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 2i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:916:1, end:916:2))"]
#[derive(Clone, Debug)]
struct XRL_instructionVar87 {
    Direct: TableDirect,
    Data: TableData,
}
impl XRL_instructionVar87 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldoplo().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:657:1, end:657:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar88 {
    Areg: TableAreg,
    Ri: TableRi,
}
impl ADD_instructionVar88 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:662:1, end:662:2))"]
#[derive(Clone, Debug)]
struct ADDC_instructionVar89 {
    Areg: TableAreg,
    Ri: TableRi,
}
impl ADDC_instructionVar89 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:674:1, end:674:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar90 {
    Areg: TableAreg,
    Ri: TableRi,
}
impl ANL_instructionVar90 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:695:1, end:695:2))"]
#[derive(Clone, Debug)]
struct CJNE_instructionVar91 {
    Ri: TableRi,
    Data: TableData,
    Rel8: TableRel8,
}
impl CJNE_instructionVar91 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:727:1, end:727:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar92 {
    Ri: TableRi,
}
impl DEC_instructionVar92 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:746:1, end:746:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar93 {
    Ri: TableRi,
}
impl INC_instructionVar93 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:790:1, end:790:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar94 {
    Areg: TableAreg,
    Ri: TableRi,
}
impl MOV_instructionVar94 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:798:1, end:798:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar95 {
    Ri: TableRi,
    Direct: TableDirect,
}
impl MOV_instructionVar95 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:800:1, end:800:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar96 {
    Ri: TableRi,
    Areg: TableAreg,
}
impl MOV_instructionVar96 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:801:1, end:801:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar97 {
    Ri: TableRi,
    Direct: TableDirect,
}
impl MOV_instructionVar97 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:802:1, end:802:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar98 {
    Ri: TableRi,
    Data: TableData,
}
impl MOV_instructionVar98 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:825:1, end:825:2))"]
#[derive(Clone, Debug)]
struct MOVX_instructionVar99 {
    RiX: TableRiX,
    Areg: TableAreg,
}
impl MOVX_instructionVar99 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal(" "),
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 1i128 {
            return None;
        }
        let RiX = if let Some((len, table)) =
            TableRiX::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:827:1, end:827:2))"]
#[derive(Clone, Debug)]
struct MOVX_instructionVar100 {
    RiX: TableRiX,
    Areg: TableAreg,
}
impl MOVX_instructionVar100 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVX"),
            DisplayElement::Literal(" "),
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 1i128 {
            return None;
        }
        let RiX = if let Some((len, table)) =
            TableRiX::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:837:1, end:837:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar101 {
    Areg: TableAreg,
    Ri: TableRi,
}
impl ORL_instructionVar101 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:899:1, end:899:2))"]
#[derive(Clone, Debug)]
struct SUBB_instructionVar102 {
    Areg: TableAreg,
    Ri: TableRi,
}
impl SUBB_instructionVar102 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:906:1, end:906:2))"]
#[derive(Clone, Debug)]
struct XCH_instructionVar103 {
    Ri: TableRi,
    Areg: TableAreg,
}
impl XCH_instructionVar103 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XCH"), DisplayElement::Literal(" ")];
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:909:1, end:909:2))"]
#[derive(Clone, Debug)]
struct XCHD_instructionVar104 {
    Areg: TableAreg,
    Ri: TableRi,
}
impl XCHD_instructionVar104 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:913:1, end:913:2))"]
#[derive(Clone, Debug)]
struct XRL_instructionVar105 {
    Ri: TableRi,
    Areg: TableAreg,
}
impl XRL_instructionVar105 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldrifill().disassembly() != 3i128 {
            return None;
        }
        let Ri = if let Some((len, table)) =
            TableRi::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:655:1, end:655:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar106 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl ADD_instructionVar106 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 2i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:660:1, end:660:2))"]
#[derive(Clone, Debug)]
struct ADDC_instructionVar107 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl ADDC_instructionVar107 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 3i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:669:1, end:669:2))"]
#[derive(Clone, Debug)]
struct AJMP_instructionVar108 {
    Addr11: TableAddr11,
}
impl AJMP_instructionVar108 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("AJMP"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr11.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldaaddrfill().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldaoplo().disassembly() != 1i128 {
            return None;
        }
        let Addr11 = if let Some((len, table)) = TableAddr11::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:672:1, end:672:2))"]
#[derive(Clone, Debug)]
struct ANL_instructionVar109 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl ANL_instructionVar109 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 5i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:694:1, end:694:2))"]
#[derive(Clone, Debug)]
struct CJNE_instructionVar110 {
    rn: TokenField_rn,
    Data: TableData,
    Rel8: TableRel8,
}
impl CJNE_instructionVar110 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 11i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:725:1, end:725:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar111 {
    rn: TokenField_rn,
}
impl DEC_instructionVar111 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:732:1, end:732:2))"]
#[derive(Clone, Debug)]
struct DJNZ_instructionVar112 {
    rn: TokenField_rn,
    Rel8: TableRel8,
}
impl DJNZ_instructionVar112 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 13i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:736:1, end:736:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar113 {
    rn: TokenField_rn,
}
impl INC_instructionVar113 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:788:1, end:788:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar114 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl MOV_instructionVar114 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 14i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:792:1, end:792:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar115 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl MOV_instructionVar115 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 15i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:793:1, end:793:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar116 {
    rn: TokenField_rn,
    Direct: TableDirect,
}
impl MOV_instructionVar116 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 10i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:794:1, end:794:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar117 {
    rn: TokenField_rn,
    Data: TableData,
}
impl MOV_instructionVar117 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 7i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Data = if let Some((len, table)) =
            TableData::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:796:1, end:796:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar118 {
    rn: TokenField_rn,
    Direct: TableDirect,
}
impl MOV_instructionVar118 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 8i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let rn = token_parser.TokenFieldrn();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let Direct = if let Some((len, table)) = TableDirect::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_1_len = block_1_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:835:1, end:835:2))"]
#[derive(Clone, Debug)]
struct ORL_instructionVar119 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl ORL_instructionVar119 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 4i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:897:1, end:897:2))"]
#[derive(Clone, Debug)]
struct SUBB_instructionVar120 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl SUBB_instructionVar120 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 9i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:904:1, end:904:2))"]
#[derive(Clone, Debug)]
struct XCH_instructionVar121 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl XCH_instructionVar121 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("XCH"), DisplayElement::Literal(" ")];
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 12i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:911:1, end:911:2))"]
#[derive(Clone, Debug)]
struct XRL_instructionVar122 {
    rn: TokenField_rn,
    Areg: TableAreg,
}
impl XRL_instructionVar122 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldophi().disassembly() != 6i128 {
            return None;
        }
        if token_parser.TokenFieldrnfill().disassembly() != 1i128 {
            return None;
        }
        let Areg = if let Some((len, table)) =
            TableAreg::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
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
    Var0(ACALL_instructionVar0),
    Var1(ANL_instructionVar1),
    Var2(ANL_instructionVar2),
    Var3(CLR_instructionVar3),
    Var4(CPL_instructionVar4),
    Var5(JB_instructionVar5),
    Var6(JBC_instructionVar6),
    Var7(JNB_instructionVar7),
    Var8(MOV_instructionVar8),
    Var9(MOV_instructionVar9),
    Var10(ORL_instructionVar10),
    Var11(ORL_instructionVar11),
    Var12(SETB_instructionVar12),
    Var13(ADD_instructionVar13),
    Var14(ADD_instructionVar14),
    Var15(ADDC_instructionVar15),
    Var16(ADDC_instructionVar16),
    Var17(ANL_instructionVar17),
    Var18(ANL_instructionVar18),
    Var19(ANL_instructionVar19),
    Var20(ANL_instructionVar20),
    Var21(ANL_instructionVar21),
    Var22(ANL_instructionVar22),
    Var23(CJNE_instructionVar23),
    Var24(CJNE_instructionVar24),
    Var25(CLR_instructionVar25),
    Var26(CLR_instructionVar26),
    Var27(CLR_instructionVar27),
    Var28(CPL_instructionVar28),
    Var29(CPL_instructionVar29),
    Var30(CPL_instructionVar30),
    Var31(DA_instructionVar31),
    Var32(DEC_instructionVar32),
    Var33(DEC_instructionVar33),
    Var34(DIV_instructionVar34),
    Var35(DJNZ_instructionVar35),
    Var36(INC_instructionVar36),
    Var37(INC_instructionVar37),
    Var38(INC_instructionVar38),
    Var39(JB_instructionVar39),
    Var40(JBC_instructionVar40),
    Var41(JC_instructionVar41),
    Var42(JMP_instructionVar42),
    Var43(JNB_instructionVar43),
    Var44(JNC_instructionVar44),
    Var45(JNZ_instructionVar45),
    Var46(JZ_instructionVar46),
    Var47(LCALL_instructionVar47),
    Var48(LJMP_instructionVar48),
    Var49(MOV_instructionVar49),
    Var50(MOV_instructionVar50),
    Var51(MOV_instructionVar51),
    Var52(MOV_instructionVar52),
    Var53(MOV_instructionVar53),
    Var54(MOV_instructionVar54),
    Var55(MOV_instructionVar55),
    Var56(MOV_instructionVar56),
    Var57(MOVC_instructionVar57),
    Var58(MOVC_instructionVar58),
    Var59(MOVX_instructionVar59),
    Var60(MOVX_instructionVar60),
    Var61(MUL_instructionVar61),
    Var62(NOP_instructionVar62),
    Var63(ORL_instructionVar63),
    Var64(ORL_instructionVar64),
    Var65(ORL_instructionVar65),
    Var66(ORL_instructionVar66),
    Var67(ORL_instructionVar67),
    Var68(ORL_instructionVar68),
    Var69(POP_instructionVar69),
    Var70(PUSH_instructionVar70),
    Var71(RET_instructionVar71),
    Var72(RETI_instructionVar72),
    Var73(RL_instructionVar73),
    Var74(RLC_instructionVar74),
    Var75(RR_instructionVar75),
    Var76(RRC_instructionVar76),
    Var77(SETB_instructionVar77),
    Var78(SETB_instructionVar78),
    Var79(SJMP_instructionVar79),
    Var80(SUBB_instructionVar80),
    Var81(SUBB_instructionVar81),
    Var82(SWAP_instructionVar82),
    Var83(XCH_instructionVar83),
    Var84(XRL_instructionVar84),
    Var85(XRL_instructionVar85),
    Var86(XRL_instructionVar86),
    Var87(XRL_instructionVar87),
    Var88(ADD_instructionVar88),
    Var89(ADDC_instructionVar89),
    Var90(ANL_instructionVar90),
    Var91(CJNE_instructionVar91),
    Var92(DEC_instructionVar92),
    Var93(INC_instructionVar93),
    Var94(MOV_instructionVar94),
    Var95(MOV_instructionVar95),
    Var96(MOV_instructionVar96),
    Var97(MOV_instructionVar97),
    Var98(MOV_instructionVar98),
    Var99(MOVX_instructionVar99),
    Var100(MOVX_instructionVar100),
    Var101(ORL_instructionVar101),
    Var102(SUBB_instructionVar102),
    Var103(XCH_instructionVar103),
    Var104(XCHD_instructionVar104),
    Var105(XRL_instructionVar105),
    Var106(ADD_instructionVar106),
    Var107(ADDC_instructionVar107),
    Var108(AJMP_instructionVar108),
    Var109(ANL_instructionVar109),
    Var110(CJNE_instructionVar110),
    Var111(DEC_instructionVar111),
    Var112(DJNZ_instructionVar112),
    Var113(INC_instructionVar113),
    Var114(MOV_instructionVar114),
    Var115(MOV_instructionVar115),
    Var116(MOV_instructionVar116),
    Var117(MOV_instructionVar117),
    Var118(MOV_instructionVar118),
    Var119(ORL_instructionVar119),
    Var120(SUBB_instructionVar120),
    Var121(XCH_instructionVar121),
    Var122(XRL_instructionVar122),
}
impl Tableinstruction {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) = ACALL_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = CLR_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = CPL_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = JB_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = JBC_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = JNB_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = SETB_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDC_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDC_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = CJNE_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = CJNE_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = CLR_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = CLR_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = CLR_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = CPL_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = CPL_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = CPL_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = DA_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = DIV_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = DJNZ_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = JB_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = JBC_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = JC_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = JMP_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = JNB_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = JNC_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = JNZ_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = JZ_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = LCALL_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = LJMP_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVC_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVC_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVX_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVX_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = MUL_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = NOP_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = POP_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = PUSH_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = RET_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = RETI_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = RL_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = RLC_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = RR_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = RRC_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = SETB_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = SETB_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = SJMP_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBB_instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBB_instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = SWAP_instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = XCH_instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = XRL_instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = XRL_instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = XRL_instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = XRL_instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDC_instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = CJNE_instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var93(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var94(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var95(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var96(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var97(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVX_instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = MOVX_instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBB_instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = XCH_instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = XCHD_instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = XRL_instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = ADDC_instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = AJMP_instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = ANL_instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        if let Some((inst_len, parsed)) = CJNE_instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = DJNZ_instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = MOV_instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = ORL_instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = SUBB_instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = XCH_instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = XRL_instructionVar122::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:507:1, end:507:3))"]
#[derive(Clone, Debug)]
struct CYVar0 {}
impl CYVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:509:1, end:509:5))"]
#[derive(Clone, Debug)]
struct AregVar0 {}
impl AregVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:510:1, end:510:6))"]
#[derive(Clone, Debug)]
struct ABregVar0 {}
impl ABregVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:511:1, end:511:8))"]
#[derive(Clone, Debug)]
struct DPTRregVar0 {}
impl DPTRregVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:516:1, end:516:6))"]
#[derive(Clone, Debug)]
struct ADPTRVar0 {}
impl ADPTRVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:523:1, end:523:4))"]
#[derive(Clone, Debug)]
struct APCVar0 {}
impl APCVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 0u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:528:1, end:528:7))"]
#[derive(Clone, Debug)]
struct ATDPTRVar0 {}
impl ATDPTRVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:540:1, end:540:3))"]
#[derive(Clone, Debug)]
struct RiVar0 {
    ri: TokenField_ri,
}
impl RiVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:546:1, end:546:4))"]
#[derive(Clone, Debug)]
struct RiXVar0 {
    ri: TokenField_ri,
}
impl RiXVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:553:1, end:553:5))"]
#[derive(Clone, Debug)]
struct DataVar0 {
    data: TokenField_data,
}
impl DataVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:554:1, end:554:7))"]
#[derive(Clone, Debug)]
struct Data16Var0 {
    data16: TokenField_data16,
}
impl Data16Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:567:1, end:567:7))"]
#[derive(Clone, Debug)]
struct DirectVar0 {}
impl DirectVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 208i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:568:1, end:568:7))"]
#[derive(Clone, Debug)]
struct DirectVar1 {}
impl DirectVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 224i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:569:1, end:569:7))"]
#[derive(Clone, Debug)]
struct DirectVar2 {}
impl DirectVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 240i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:570:1, end:570:7))"]
#[derive(Clone, Debug)]
struct DirectVar3 {}
impl DirectVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 130i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:571:1, end:571:7))"]
#[derive(Clone, Debug)]
struct DirectVar4 {}
impl DirectVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect().disassembly() != 131i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:564:1, end:564:7))"]
#[derive(Clone, Debug)]
struct DirectVar5 {
    mainreg: TokenField_mainreg,
}
impl DirectVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 0i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:566:1, end:566:7))"]
#[derive(Clone, Debug)]
struct DirectVar6 {
    direct: TokenField_direct,
}
impl DirectVar6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank().disassembly() != 1i128 {
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
}
impl TableDirect {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:587:1, end:587:8))"]
#[derive(Clone, Debug)]
struct Direct2Var0 {}
impl Direct2Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 208i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:588:1, end:588:8))"]
#[derive(Clone, Debug)]
struct Direct2Var1 {}
impl Direct2Var1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 224i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:589:1, end:589:8))"]
#[derive(Clone, Debug)]
struct Direct2Var2 {}
impl Direct2Var2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 240i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:590:1, end:590:8))"]
#[derive(Clone, Debug)]
struct Direct2Var3 {}
impl Direct2Var3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 130i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:591:1, end:591:8))"]
#[derive(Clone, Debug)]
struct Direct2Var4 {}
impl Direct2Var4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFielddirect2().disassembly() != 131i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:584:1, end:584:8))"]
#[derive(Clone, Debug)]
struct Direct2Var5 {
    mainreg2: TokenField_mainreg2,
}
impl Direct2Var5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 0i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:586:1, end:586:8))"]
#[derive(Clone, Debug)]
struct Direct2Var6 {
    direct2: TokenField_direct2,
}
impl Direct2Var6 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbank2().disassembly() != 1i128 {
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
}
impl TableDirect2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        }
    }
    fn parse<T>(
        tokens_param: &[u8],
        context_param: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:611:1, end:611:8))"]
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
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_bitaddr: i128 = 0;
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| self.sfrbyte.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_bitaddr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_bitaddr: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i128 {
            return None;
        }
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldsfrbyte()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:612:1, end:612:8))"]
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
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_bitaddr: i128 = 0;
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| self.lowbyte.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_bitaddr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_bitaddr: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 0i128 {
            return None;
        }
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldlowbyte()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:613:1, end:613:9))"]
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
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_bitaddr: i128 = 0;
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| self.sfrbyte.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("/"),
            DisplayElement::Number(true, calc_bitaddr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_bitaddr: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i128 {
            return None;
        }
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldsfrbyte()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:614:1, end:614:9))"]
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
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_bitaddr: i128 = 0;
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| self.lowbyte.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0)
            .wrapping_add(self.sfrbit.disassembly());
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("/"),
            DisplayElement::Number(true, calc_bitaddr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_bitaddr: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 0i128 {
            return None;
        }
        calc_bitaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldlowbyte()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:618:1, end:618:12))"]
#[derive(Clone, Debug)]
struct BitByteAddrVar0 {}
impl BitByteAddrVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbyte().disassembly() != 28i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:619:1, end:619:12))"]
#[derive(Clone, Debug)]
struct BitByteAddrVar1 {}
impl BitByteAddrVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbyte().disassembly() != 30i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:620:1, end:620:12))"]
#[derive(Clone, Debug)]
struct BitByteAddrVar2 {}
impl BitByteAddrVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldsfrbyte().disassembly() != 26i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:617:1, end:617:12))"]
#[derive(Clone, Debug)]
struct BitByteAddrVar3 {
    sfrbyte: TokenField_sfrbyte,
}
impl BitByteAddrVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_byteaddr: i128 = 0;
        calc_byteaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| self.sfrbyte.disassembly().checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_byteaddr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_byteaddr: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 1i128 {
            return None;
        }
        calc_byteaddr = u32::try_from(3i128)
            .ok()
            .map(|shl| {
                token_parser
                    .TokenFieldsfrbyte()
                    .disassembly()
                    .checked_shl(shl)
            })
            .flatten()
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:626:1, end:626:12))"]
#[derive(Clone, Debug)]
struct BitByteAddrVar4 {
    lowbyte: TokenField_lowbyte,
}
impl BitByteAddrVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_byteaddr: i128 = 0;
        calc_byteaddr = self.lowbyte.disassembly().wrapping_add(32i128);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_byteaddr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_byteaddr: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        if token_parser.TokenFieldbitbank().disassembly() != 0i128 {
            return None;
        }
        calc_byteaddr = token_parser
            .TokenFieldlowbyte()
            .disassembly()
            .wrapping_add(32i128);
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:633:1, end:633:7))"]
#[derive(Clone, Debug)]
struct Addr11Var0 {
    aopaddr: TokenField_aopaddr,
    adata: TokenField_adata,
}
impl Addr11Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_relAddr: i128 = 0;
        calc_relAddr = (i128::try_from(inst_next).unwrap() & 63488i128)
            .wrapping_add(self.aopaddr.disassembly().wrapping_mul(256i128))
            .wrapping_add(self.adata.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_relAddr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_relAddr: i128 = 0;
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
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
enum TableAddr11 {
    Var0(Addr11Var0),
}
impl TableAddr11 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            Addr11Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:634:1, end:634:7))"]
#[derive(Clone, Debug)]
struct Addr16Var0 {
    addr16: TokenField_addr16,
}
impl Addr16Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.addr16.display()];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let addr16 = token_parser.TokenFieldaddr16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr16 }))
    }
}
#[derive(Clone, Debug)]
enum TableAddr16 {
    Var0(Addr16Var0),
}
impl TableAddr16 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_len, parsed)) =
            Addr16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:640:1, end:640:5))"]
#[derive(Clone, Debug)]
struct Rel8Var0 {
    rel8: TokenField_rel8,
}
impl Rel8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_relAddr: i128 = 0;
        calc_relAddr = i128::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.rel8.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_relAddr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_relAddr: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8051/data/languages/8051_main.sinc, start:641:1, end:641:6))"]
#[derive(Clone, Debug)]
struct Rel16Var0 {
    rel16: TokenField_rel16,
}
impl Rel16Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_relAddr: i128 = 0;
        calc_relAddr = i128::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.rel16.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_relAddr)];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u16,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u16;
        let mut context_instance = context.clone();
        let mut calc_relAddr: i128 = 0;
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
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
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
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
        inst_start: u16,
    ) -> Option<(u16, Self)>
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
    inst_start: u16,
    global_set: &mut impl GlobalSetTrait,
) -> Option<(u16, Vec<DisplayElement>)>
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
