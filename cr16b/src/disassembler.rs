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
#[derive(Clone, Copy, Debug)]
struct TokenField_b0(u8);
impl TokenField_b0 {
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
struct TokenField_op1(u8);
impl TokenField_op1 {
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
struct TokenField_op2(u8);
impl TokenField_op2 {
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
struct TokenField_opcode1(u8);
impl TokenField_opcode1 {
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
struct TokenField_i(u8);
impl TokenField_i {
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
struct TokenField_opcode2(u8);
impl TokenField_opcode2 {
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
struct TokenField_op1_b02(u8);
impl TokenField_op1_b02 {
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
struct TokenField_op1_b12(u8);
impl TokenField_op1_b12 {
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
struct TokenField_op2_b23(u8);
impl TokenField_op2_b23 {
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
struct TokenField_op2_b12(u8);
impl TokenField_op2_b12 {
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
struct TokenField_opcode1_b23(u8);
impl TokenField_opcode1_b23 {
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
struct TokenField_opcode1_b13(u8);
impl TokenField_opcode1_b13 {
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
    fn TokenFieldb0(&self) -> TokenField_b0 {
        let inner_value = self.read_u8::<false>(0, 0, 1).unwrap();
        TokenField_b0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1(&self) -> TokenField_op1 {
        let inner_value = self.read_u8::<false>(0, 1, 4).unwrap();
        TokenField_op1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2(&self) -> TokenField_op2 {
        let inner_value = self.read_u16::<false>(0, 5, 4).unwrap();
        TokenField_op2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode1(&self) -> TokenField_opcode1 {
        let inner_value = self.read_u8::<false>(1, 1, 4).unwrap();
        TokenField_opcode1(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldi(&self) -> TokenField_i {
        let inner_value = self.read_u8::<false>(1, 5, 1).unwrap();
        TokenField_i(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode2(&self) -> TokenField_opcode2 {
        let inner_value = self.read_u8::<false>(1, 6, 2).unwrap();
        TokenField_opcode2(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1_b02(&self) -> TokenField_op1_b02 {
        let inner_value = self.read_u8::<false>(0, 1, 3).unwrap();
        TokenField_op1_b02(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop1_b12(&self) -> TokenField_op1_b12 {
        let inner_value = self.read_u8::<false>(0, 2, 2).unwrap();
        TokenField_op1_b12(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2_b23(&self) -> TokenField_op2_b23 {
        let inner_value = self.read_u16::<false>(0, 7, 2).unwrap();
        TokenField_op2_b23(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop2_b12(&self) -> TokenField_op2_b12 {
        let inner_value = self.read_u8::<false>(0, 6, 2).unwrap();
        TokenField_op2_b12(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode1_b23(&self) -> TokenField_opcode1_b23 {
        let inner_value = self.read_u8::<false>(1, 3, 2).unwrap();
        TokenField_opcode1_b23(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldopcode1_b13(&self) -> TokenField_opcode1_b13 {
        let inner_value = self.read_u8::<false>(1, 2, 3).unwrap();
        TokenField_opcode1_b13(u8::try_from(inner_value).unwrap())
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
    R8,
    R9,
    R10,
    R11,
    R12_L,
    R12_H,
    R13_L,
    R13_H,
    RA_L,
    RA_H,
    SP_L,
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
            Self::R8 => write!(f, "R8"),
            Self::R9 => write!(f, "R9"),
            Self::R10 => write!(f, "R10"),
            Self::R11 => write!(f, "R11"),
            Self::R12_L => write!(f, "R12_L"),
            Self::R12_H => write!(f, "R12_H"),
            Self::R13_L => write!(f, "R13_L"),
            Self::R13_H => write!(f, "R13_H"),
            Self::RA_L => write!(f, "RA_L"),
            Self::RA_H => write!(f, "RA_H"),
            Self::SP_L => write!(f, "SP_L"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:310:1"]
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
            [DisplayElement::Literal("RETX")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:379:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DI")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:382:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("EI")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:385:1"]
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
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:388:1"]
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
            [DisplayElement::Literal("WAIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:391:1"]
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
            [DisplayElement::Literal("EIWAIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:278:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("BR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldop1_b02().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:346:1"]
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
            [DisplayElement::Literal("LOADM")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:372:1"]
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORM")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b23().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:261:1"]
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
            [DisplayElement::Literal("Bcond")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop1_b02().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:268:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("BAL")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop1_b02().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:124:1"]
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
            [DisplayElement::Literal("MULUW")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldop1_b12().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:281:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("BR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:285:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("EXCP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:303:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("JUMP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:306:1"]
#[derive(Clone, Debug)]
struct instructionVar15 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("JUMP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:314:1"]
#[derive(Clone, Debug)]
struct instructionVar16 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("PUSH")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b23().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:318:1"]
#[derive(Clone, Debug)]
struct instructionVar17 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("POP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b23().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:322:1"]
#[derive(Clone, Debug)]
struct instructionVar18 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("POPRET")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b23().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:326:1"]
#[derive(Clone, Debug)]
struct instructionVar19 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("POPRET")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b23().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:76:1"]
#[derive(Clone, Debug)]
struct instructionVar20 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("MOVXB")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:79:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("MOVZB")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:118:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("MULSB")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:121:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("MULSW")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:152:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("BEQ0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:156:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("BEQ1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:160:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("BNE0")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:164:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("BNE1")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:212:1"]
#[derive(Clone, Debug)]
struct instructionVar28 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("TBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:219:1"]
#[derive(Clone, Debug)]
struct instructionVar29 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("TBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:222:1"]
#[derive(Clone, Debug)]
struct instructionVar30 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("TBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:225:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("TBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:229:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("CBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:232:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("CBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:235:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("CBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:239:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:242:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:245:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:250:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("LPR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:253:1"]
#[derive(Clone, Debug)]
struct instructionVar39 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SPR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:264:1"]
#[derive(Clone, Debug)]
struct instructionVar40 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("Bcond")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:271:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("BAL")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:289:1"]
#[derive(Clone, Debug)]
struct instructionVar42 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("Jcond")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:292:1"]
#[derive(Clone, Debug)]
struct instructionVar43 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("Jcond")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:296:1"]
#[derive(Clone, Debug)]
struct instructionVar44 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("JAL")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 10i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:299:1"]
#[derive(Clone, Debug)]
struct instructionVar45 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("JAL")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 11i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:362:1"]
#[derive(Clone, Debug)]
struct instructionVar46 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:365:1"]
#[derive(Clone, Debug)]
struct instructionVar47 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:368:1"]
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldop2_b12().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:69:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("MOV")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 12i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:89:1"]
#[derive(Clone, Debug)]
struct instructionVar50 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ADD")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:96:1"]
#[derive(Clone, Debug)]
struct instructionVar51 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ADDU")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:103:1"]
#[derive(Clone, Debug)]
struct instructionVar52 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ADDC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 9i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:111:1"]
#[derive(Clone, Debug)]
struct instructionVar53 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("MUL")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:128:1"]
#[derive(Clone, Debug)]
struct instructionVar54 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SUB")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:135:1"]
#[derive(Clone, Debug)]
struct instructionVar55 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SUBC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 13i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:145:1"]
#[derive(Clone, Debug)]
struct instructionVar56 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CMP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:171:1"]
#[derive(Clone, Debug)]
struct instructionVar57 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("AND")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 8i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:178:1"]
#[derive(Clone, Debug)]
struct instructionVar58 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("OR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:185:1"]
#[derive(Clone, Debug)]
struct instructionVar59 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("S")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:189:1"]
#[derive(Clone, Debug)]
struct instructionVar60 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("XOR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:197:1"]
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
            [DisplayElement::Literal("ASHU")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:204:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("LSH")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:215:1"]
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
            [DisplayElement::Literal("TBIT")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 11i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:72:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("MOV")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 12i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:82:1"]
#[derive(Clone, Debug)]
struct instructionVar65 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("MOVD")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1_b13().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:92:1"]
#[derive(Clone, Debug)]
struct instructionVar66 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ADD")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 0i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:99:1"]
#[derive(Clone, Debug)]
struct instructionVar67 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ADDU")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:106:1"]
#[derive(Clone, Debug)]
struct instructionVar68 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ADDC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 9i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:114:1"]
#[derive(Clone, Debug)]
struct instructionVar69 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("MUL")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 3i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:131:1"]
#[derive(Clone, Debug)]
struct instructionVar70 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SUB")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 15i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:138:1"]
#[derive(Clone, Debug)]
struct instructionVar71 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SUBC")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 13i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:148:1"]
#[derive(Clone, Debug)]
struct instructionVar72 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CMP")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 7i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:174:1"]
#[derive(Clone, Debug)]
struct instructionVar73 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("AND")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 8i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:181:1"]
#[derive(Clone, Debug)]
struct instructionVar74 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("OR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 14i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:192:1"]
#[derive(Clone, Debug)]
struct instructionVar75 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("XOR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 6i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:200:1"]
#[derive(Clone, Debug)]
struct instructionVar76 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ASHU")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 4i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:207:1"]
#[derive(Clone, Debug)]
struct instructionVar77 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("LSH")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1().disassembly() != 5i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:275:1"]
#[derive(Clone, Debug)]
struct instructionVar78 {}
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("BR")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldop2().disassembly() != 14i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let opcode1 = token_parser.TokenFieldopcode1();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:258:1"]
#[derive(Clone, Debug)]
struct instructionVar79 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("Bcond")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldi().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 0i64 {
            return None;
        }
        let opcode1 = token_parser.TokenFieldopcode1();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:342:1"]
#[derive(Clone, Debug)]
struct instructionVar80 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("LOAD")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1_b23().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:359:1"]
#[derive(Clone, Debug)]
struct instructionVar81 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1_b23().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldop1().disassembly() != 15i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:336:1"]
#[derive(Clone, Debug)]
struct instructionVar82 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("LOAD")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1_b23().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:339:1"]
#[derive(Clone, Debug)]
struct instructionVar83 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("LOAD")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1_b23().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:353:1"]
#[derive(Clone, Debug)]
struct instructionVar84 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1_b23().disassembly() != 2i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:356:1"]
#[derive(Clone, Debug)]
struct instructionVar85 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldopcode1_b23().disassembly() != 3i64 {
            return None;
        }
        if token_parser.TokenFieldb0().disassembly() != 1i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:333:1"]
#[derive(Clone, Debug)]
struct instructionVar86 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("LOAD")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 2i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let opcode1 = token_parser.TokenFieldopcode1();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/CR16/data/languages/CR16B.sinc:350:1"]
#[derive(Clone, Debug)]
struct instructionVar87 {}
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
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STORE")];
        display.extend_from_slice(&extend);
    }
    fn parse<T>(
        mut tokens_current: &[u8],
        context: &mut T,
        inst_start: u32,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut pattern_len = 0 as u32;
        let mut context_instance = context.clone();
        let mut block_0_len = 2u64 as u32;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldopcode2().disassembly() != 3i64 {
            return None;
        }
        let i = token_parser.TokenFieldi();
        let opcode1 = token_parser.TokenFieldopcode1();
        let op2 = token_parser.TokenFieldop2();
        let op1 = token_parser.TokenFieldop1();
        let b0 = token_parser.TokenFieldb0();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
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
        }
    }
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
