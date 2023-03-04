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
#[derive(Clone, Copy, Debug)]
struct TokenField_op(u8);
impl TokenField_op {
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
struct TokenField_aaa(u8);
impl TokenField_aaa {
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
struct TokenField_bbb(u8);
impl TokenField_bbb {
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
struct TokenField_cc(u8);
impl TokenField_cc {
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
struct TokenField_imm8(u8);
impl TokenField_imm8 {
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
struct TokenField_rel(i8);
impl TokenField_rel {
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
struct TokenField_imm16(u16);
impl TokenField_imm16 {
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
    fn TokenFieldop(&self) -> TokenField_op {
        let inner_value = self.read_u8::<false>(0, 0, 8).unwrap();
        TokenField_op(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldaaa(&self) -> TokenField_aaa {
        let inner_value = self.read_u8::<false>(0, 5, 3).unwrap();
        TokenField_aaa(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldbbb(&self) -> TokenField_bbb {
        let inner_value = self.read_u8::<false>(0, 2, 3).unwrap();
        TokenField_bbb(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldcc(&self) -> TokenField_cc {
        let inner_value = self.read_u8::<false>(0, 0, 2).unwrap();
        TokenField_cc(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm8(&self) -> TokenField_imm8 {
        let inner_value = self.read_u8::<false>(0, 0, 8).unwrap();
        TokenField_imm8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrel(&self) -> TokenField_rel {
        let inner_value = self.read_i8::<false>(0, 0, 8).unwrap();
        TokenField_rel(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm16(&self) -> TokenField_imm16 {
        let inner_value = self.read_u16::<false>(0, 0, 16).unwrap();
        TokenField_imm16(u16::try_from(inner_value).unwrap())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    A,
    X,
    Y,
    P,
    PC,
    SP,
    PCL,
    PCH,
    S,
    SH,
    N,
    V,
    B,
    D,
    I,
    Z,
    C,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
            Self::P => write!(f, "P"),
            Self::PC => write!(f, "PC"),
            Self::SP => write!(f, "SP"),
            Self::PCL => write!(f, "PCL"),
            Self::PCH => write!(f, "PCH"),
            Self::S => write!(f, "S"),
            Self::SH => write!(f, "SH"),
            Self::N => write!(f, "N"),
            Self::V => write!(f, "V"),
            Self::B => write!(f, "B"),
            Self::D => write!(f, "D"),
            Self::I => write!(f, "I"),
            Self::Z => write!(f, "Z"),
            Self::C => write!(f, "C"),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:155:1, end:155:2))"]
#[derive(Clone, Debug)]
struct BCC_instructionVar0 {
    REL: TableREL,
}
impl BCC_instructionVar0 {
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
            [DisplayElement::Literal("BCC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 144i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:160:1, end:160:2))"]
#[derive(Clone, Debug)]
struct BCS_instructionVar1 {
    REL: TableREL,
}
impl BCS_instructionVar1 {
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
            [DisplayElement::Literal("BCS"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 176i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:165:1, end:165:2))"]
#[derive(Clone, Debug)]
struct BEQ_instructionVar2 {
    REL: TableREL,
}
impl BEQ_instructionVar2 {
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
            [DisplayElement::Literal("BEQ"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 240i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:178:1, end:178:2))"]
#[derive(Clone, Debug)]
struct BMI_instructionVar3 {
    REL: TableREL,
}
impl BMI_instructionVar3 {
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
            [DisplayElement::Literal("BMI"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 48i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:183:1, end:183:2))"]
#[derive(Clone, Debug)]
struct BNE_instructionVar4 {
    REL: TableREL,
}
impl BNE_instructionVar4 {
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
            [DisplayElement::Literal("BNE"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 208i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:188:1, end:188:2))"]
#[derive(Clone, Debug)]
struct BPL_instructionVar5 {
    REL: TableREL,
}
impl BPL_instructionVar5 {
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
            [DisplayElement::Literal("BPL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 16i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:193:1, end:193:2))"]
#[derive(Clone, Debug)]
struct BRK_instructionVar6 {}
impl BRK_instructionVar6 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("BRK")];
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
        if token_parser.TokenFieldop().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct BVC_instructionVar7 {
    REL: TableREL,
}
impl BVC_instructionVar7 {
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
            [DisplayElement::Literal("BVC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 80i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:209:1, end:209:2))"]
#[derive(Clone, Debug)]
struct BVS_instructionVar8 {
    REL: TableREL,
}
impl BVS_instructionVar8 {
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
            [DisplayElement::Literal("BVS"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 112i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:214:1, end:214:2))"]
#[derive(Clone, Debug)]
struct CLC_instructionVar9 {}
impl CLC_instructionVar9 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CLC")];
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
        if token_parser.TokenFieldop().disassembly() != 24i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:219:1, end:219:2))"]
#[derive(Clone, Debug)]
struct CLD_instructionVar10 {}
impl CLD_instructionVar10 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CLD")];
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
        if token_parser.TokenFieldop().disassembly() != 216i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:224:1, end:224:2))"]
#[derive(Clone, Debug)]
struct CLI_instructionVar11 {}
impl CLI_instructionVar11 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CLI")];
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
        if token_parser.TokenFieldop().disassembly() != 88i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:229:1, end:229:2))"]
#[derive(Clone, Debug)]
struct CLV_instructionVar12 {}
impl CLV_instructionVar12 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("CLV")];
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
        if token_parser.TokenFieldop().disassembly() != 184i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:265:1, end:265:2))"]
#[derive(Clone, Debug)]
struct DEX_instructionVar13 {}
impl DEX_instructionVar13 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DEX")];
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
        if token_parser.TokenFieldop().disassembly() != 202i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:272:1, end:272:2))"]
#[derive(Clone, Debug)]
struct DEY_instructionVar14 {}
impl DEY_instructionVar14 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DEY")];
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
        if token_parser.TokenFieldop().disassembly() != 136i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:292:1, end:292:2))"]
#[derive(Clone, Debug)]
struct INY_instructionVar15 {}
impl INY_instructionVar15 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("INY")];
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
        if token_parser.TokenFieldop().disassembly() != 200i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:298:1, end:298:2))"]
#[derive(Clone, Debug)]
struct INX_instructionVar16 {}
impl INX_instructionVar16 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("INX")];
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
        if token_parser.TokenFieldop().disassembly() != 232i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:304:1, end:304:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar17 {
    ADDR16: TableADDR16,
}
impl JMP_instructionVar17 {
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
        self.ADDR16.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 76i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let ADDR16 = if let Some((len, table)) = TableADDR16::parse(
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
        Some((pattern_len, Self { ADDR16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:309:1, end:309:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar18 {
    ADDRI: TableADDRI,
}
impl JMP_instructionVar18 {
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
        self.ADDRI.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 108i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let ADDRI = if let Some((len, table)) =
            TableADDRI::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { ADDRI }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:314:1, end:314:2))"]
#[derive(Clone, Debug)]
struct JSR_instructionVar19 {
    ADDR16: TableADDR16,
}
impl JSR_instructionVar19 {
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
            [DisplayElement::Literal("JSR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ADDR16.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 32i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let ADDR16 = if let Some((len, table)) = TableADDR16::parse(
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
        Some((pattern_len, Self { ADDR16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:349:1, end:349:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar20 {}
impl NOP_instructionVar20 {
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
        if token_parser.TokenFieldop().disassembly() != 234i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:359:1, end:359:2))"]
#[derive(Clone, Debug)]
struct PHP_instructionVar21 {}
impl PHP_instructionVar21 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PHP")];
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
        if token_parser.TokenFieldop().disassembly() != 8i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:364:1, end:364:2))"]
#[derive(Clone, Debug)]
struct PLP_instructionVar22 {}
impl PLP_instructionVar22 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PLP")];
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
        if token_parser.TokenFieldop().disassembly() != 40i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:369:1, end:369:2))"]
#[derive(Clone, Debug)]
struct PHA_instructionVar23 {}
impl PHA_instructionVar23 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PHA")];
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
        if token_parser.TokenFieldop().disassembly() != 72i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:375:1, end:375:2))"]
#[derive(Clone, Debug)]
struct PLA_instructionVar24 {}
impl PLA_instructionVar24 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PLA")];
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
        if token_parser.TokenFieldop().disassembly() != 104i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct RTI_instructionVar25 {}
impl RTI_instructionVar25 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RTI")];
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
        if token_parser.TokenFieldop().disassembly() != 64i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:415:1, end:415:2))"]
#[derive(Clone, Debug)]
struct RTS_instructionVar26 {}
impl RTS_instructionVar26 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RTS")];
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
        if token_parser.TokenFieldop().disassembly() != 96i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:437:1, end:437:2))"]
#[derive(Clone, Debug)]
struct SEC_instructionVar27 {}
impl SEC_instructionVar27 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SEC")];
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
        if token_parser.TokenFieldop().disassembly() != 56i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:442:1, end:442:2))"]
#[derive(Clone, Debug)]
struct SED_instructionVar28 {}
impl SED_instructionVar28 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SED")];
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
        if token_parser.TokenFieldop().disassembly() != 248i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:447:1, end:447:2))"]
#[derive(Clone, Debug)]
struct SEI_instructionVar29 {}
impl SEI_instructionVar29 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SEI")];
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
        if token_parser.TokenFieldop().disassembly() != 120i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:467:1, end:467:2))"]
#[derive(Clone, Debug)]
struct TAX_instructionVar30 {}
impl TAX_instructionVar30 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TAX")];
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
        if token_parser.TokenFieldop().disassembly() != 170i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:473:1, end:473:2))"]
#[derive(Clone, Debug)]
struct TAY_instructionVar31 {}
impl TAY_instructionVar31 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TAY")];
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
        if token_parser.TokenFieldop().disassembly() != 168i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:479:1, end:479:2))"]
#[derive(Clone, Debug)]
struct TSX_instructionVar32 {}
impl TSX_instructionVar32 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TSX")];
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
        if token_parser.TokenFieldop().disassembly() != 186i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:484:1, end:484:2))"]
#[derive(Clone, Debug)]
struct TXA_instructionVar33 {}
impl TXA_instructionVar33 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TXA")];
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
        if token_parser.TokenFieldop().disassembly() != 138i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:490:1, end:490:2))"]
#[derive(Clone, Debug)]
struct TXS_instructionVar34 {}
impl TXS_instructionVar34 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TXS")];
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
        if token_parser.TokenFieldop().disassembly() != 154i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:495:1, end:495:2))"]
#[derive(Clone, Debug)]
struct TYA_instructionVar35 {}
impl TYA_instructionVar35 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TYA")];
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
        if token_parser.TokenFieldop().disassembly() != 152i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:170:1, end:170:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar36 {
    OP2: TableOP2,
}
impl BIT_instructionVar36 {
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
            [DisplayElement::Literal("BIT"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 36i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 44i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:242:1, end:242:2))"]
#[derive(Clone, Debug)]
struct CPX_instructionVar37 {
    OP2: TableOP2,
}
impl CPX_instructionVar37 {
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
            [DisplayElement::Literal("CPX"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 224i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 228i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 236i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:250:1, end:250:2))"]
#[derive(Clone, Debug)]
struct CPY_instructionVar38 {
    OP2: TableOP2,
}
impl CPY_instructionVar38 {
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
            [DisplayElement::Literal("CPY"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 192i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 196i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 204i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:258:1, end:258:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar39 {
    OP2: TableOP2,
}
impl DEC_instructionVar39 {
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
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 198i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 206i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 214i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 222i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:285:1, end:285:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar40 {
    OP2: TableOP2,
}
impl INC_instructionVar40 {
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
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 230i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 238i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 246i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 254i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:457:1, end:457:2))"]
#[derive(Clone, Debug)]
struct STX_instructionVar41 {
    OP2ST: TableOP2ST,
}
impl STX_instructionVar41 {
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
            [DisplayElement::Literal("STX"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2ST.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 134i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 142i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 150i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2ST = if let Some((len, table)) =
            TableOP2ST::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2ST }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:462:1, end:462:2))"]
#[derive(Clone, Debug)]
struct STY_instructionVar42 {
    OP2: TableOP2,
}
impl STY_instructionVar42 {
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
            [DisplayElement::Literal("STY"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 132i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 140i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 148i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:127:1, end:127:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar43 {
    OP1: TableOP1,
}
impl ADC_instructionVar43 {
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
            [DisplayElement::Literal("ADC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 3i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:140:1, end:140:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar44 {
    OP1: TableOP1,
}
impl AND_instructionVar44 {
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
            [DisplayElement::Literal("AND"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 1i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:146:1, end:146:2))"]
#[derive(Clone, Debug)]
struct ASL_instructionVar45 {
    OP2: TableOP2,
}
impl ASL_instructionVar45 {
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
            [DisplayElement::Literal("ASL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 6i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 10i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 14i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 22i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 30i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar46 {
    OP1: TableOP1,
}
impl CMP_instructionVar46 {
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
            [DisplayElement::Literal("CMP"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 6i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:278:1, end:278:2))"]
#[derive(Clone, Debug)]
struct EOR_instructionVar47 {
    OP1: TableOP1,
}
impl EOR_instructionVar47 {
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
            [DisplayElement::Literal("EOR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 2i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:321:1, end:321:2))"]
#[derive(Clone, Debug)]
struct LDA_instructionVar48 {
    OP1: TableOP1,
}
impl LDA_instructionVar48 {
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
            [DisplayElement::Literal("LDA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 5i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct LDY_instructionVar49 {
    OP2: TableOP2,
}
impl LDY_instructionVar49 {
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
            [DisplayElement::Literal("LDY"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 160i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 164i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 172i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 180i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 188i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:333:1, end:333:2))"]
#[derive(Clone, Debug)]
struct LDX_instructionVar50 {
    OP2LD: TableOP2LD,
}
impl LDX_instructionVar50 {
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
            [DisplayElement::Literal("LDX"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2LD.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 162i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 166i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 174i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 182i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 190i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2LD = if let Some((len, table)) =
            TableOP2LD::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2LD }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:339:1, end:339:2))"]
#[derive(Clone, Debug)]
struct LSR_instructionVar51 {
    OP2: TableOP2,
}
impl LSR_instructionVar51 {
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
            [DisplayElement::Literal("LSR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 70i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 74i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 78i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 86i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 94i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:353:1, end:353:2))"]
#[derive(Clone, Debug)]
struct ORA_instructionVar52 {
    OP1: TableOP1,
}
impl ORA_instructionVar52 {
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
            [DisplayElement::Literal("ORA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 0i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:382:1, end:382:2))"]
#[derive(Clone, Debug)]
struct ROL_instructionVar53 {
    OP2: TableOP2,
}
impl ROL_instructionVar53 {
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
            [DisplayElement::Literal("ROL"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 38i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 42i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 46i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 54i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 62i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:393:1, end:393:2))"]
#[derive(Clone, Debug)]
struct ROR_instructionVar54 {
    OP2: TableOP2,
}
impl ROR_instructionVar54 {
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
            [DisplayElement::Literal("ROR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 102i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 106i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 110i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 118i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 126i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:424:1, end:424:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar55 {
    OP1: TableOP1,
}
impl SBC_instructionVar55 {
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
            [DisplayElement::Literal("SBC"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 7i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:452:1, end:452:2))"]
#[derive(Clone, Debug)]
struct STA_instructionVar56 {
    OP1: TableOP1,
}
impl STA_instructionVar56 {
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
            [DisplayElement::Literal("STA"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldcc().disassembly() != 1i128 {
                return None;
            }
            if token_parser.TokenFieldaaa().disassembly() != 4i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP1 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(BCC_instructionVar0),
    Var1(BCS_instructionVar1),
    Var2(BEQ_instructionVar2),
    Var3(BMI_instructionVar3),
    Var4(BNE_instructionVar4),
    Var5(BPL_instructionVar5),
    Var6(BRK_instructionVar6),
    Var7(BVC_instructionVar7),
    Var8(BVS_instructionVar8),
    Var9(CLC_instructionVar9),
    Var10(CLD_instructionVar10),
    Var11(CLI_instructionVar11),
    Var12(CLV_instructionVar12),
    Var13(DEX_instructionVar13),
    Var14(DEY_instructionVar14),
    Var15(INY_instructionVar15),
    Var16(INX_instructionVar16),
    Var17(JMP_instructionVar17),
    Var18(JMP_instructionVar18),
    Var19(JSR_instructionVar19),
    Var20(NOP_instructionVar20),
    Var21(PHP_instructionVar21),
    Var22(PLP_instructionVar22),
    Var23(PHA_instructionVar23),
    Var24(PLA_instructionVar24),
    Var25(RTI_instructionVar25),
    Var26(RTS_instructionVar26),
    Var27(SEC_instructionVar27),
    Var28(SED_instructionVar28),
    Var29(SEI_instructionVar29),
    Var30(TAX_instructionVar30),
    Var31(TAY_instructionVar31),
    Var32(TSX_instructionVar32),
    Var33(TXA_instructionVar33),
    Var34(TXS_instructionVar34),
    Var35(TYA_instructionVar35),
    Var36(BIT_instructionVar36),
    Var37(CPX_instructionVar37),
    Var38(CPY_instructionVar38),
    Var39(DEC_instructionVar39),
    Var40(INC_instructionVar40),
    Var41(STX_instructionVar41),
    Var42(STY_instructionVar42),
    Var43(ADC_instructionVar43),
    Var44(AND_instructionVar44),
    Var45(ASL_instructionVar45),
    Var46(CMP_instructionVar46),
    Var47(EOR_instructionVar47),
    Var48(LDA_instructionVar48),
    Var49(LDY_instructionVar49),
    Var50(LDX_instructionVar50),
    Var51(LSR_instructionVar51),
    Var52(ORA_instructionVar52),
    Var53(ROL_instructionVar53),
    Var54(ROR_instructionVar54),
    Var55(SBC_instructionVar55),
    Var56(STA_instructionVar56),
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
        if let Some((inst_len, parsed)) = BCC_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = BCS_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = BEQ_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = BMI_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = BNE_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = BPL_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = BRK_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = BVC_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = BVS_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = CLC_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = CLD_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = CLI_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = CLV_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = DEX_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = DEY_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = INY_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = INX_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = JMP_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = JMP_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = JSR_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = NOP_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = PHP_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = PLP_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = PHA_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = PLA_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = RTI_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = RTS_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = SEC_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = SED_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = SEI_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = TAX_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = TAY_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = TSX_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = TXA_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = TXS_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = TYA_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = CPX_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = CPY_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = STX_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = STY_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = ASL_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = CMP_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = EOR_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = LDA_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = LDY_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = LDX_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = LSR_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = ORA_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = ROL_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = ROR_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = STA_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:80:1, end:80:4))"]
#[derive(Clone, Debug)]
struct RELVar0 {
    rel: TokenField_rel,
}
impl RELVar0 {
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
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_next)
            .unwrap()
            .wrapping_add(self.rel.disassembly());
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_reloc)];
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
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let rel = token_parser.TokenFieldrel();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rel }))
    }
}
#[derive(Clone, Debug)]
enum TableREL {
    Var0(RELVar0),
}
impl TableREL {
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
            RELVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:83:1, end:83:4))"]
#[derive(Clone, Debug)]
struct OP1Var0 {
    imm8: TokenField_imm8,
}
impl OP1Var0 {
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
            [DisplayElement::Literal("#"), self.imm8.display()];
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
        if token_parser.TokenFieldbbb().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:85:1, end:85:4))"]
#[derive(Clone, Debug)]
struct OP1Var1 {
    imm8: TokenField_imm8,
}
impl OP1Var1 {
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
        let extend: [DisplayElement; 1usize] = [self.imm8.display()];
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
        if token_parser.TokenFieldbbb().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:87:1, end:87:4))"]
#[derive(Clone, Debug)]
struct OP1Var2 {
    imm8: TokenField_imm8,
}
impl OP1Var2 {
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
            self.imm8.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::X),
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
        if token_parser.TokenFieldbbb().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:89:1, end:89:4))"]
#[derive(Clone, Debug)]
struct OP1Var3 {
    imm16: TokenField_imm16,
}
impl OP1Var3 {
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
        let extend: [DisplayElement; 1usize] = [self.imm16.display()];
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
        if token_parser.TokenFieldbbb().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:91:1, end:91:4))"]
#[derive(Clone, Debug)]
struct OP1Var4 {
    imm16: TokenField_imm16,
}
impl OP1Var4 {
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
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::X),
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
        if token_parser.TokenFieldbbb().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:93:1, end:93:4))"]
#[derive(Clone, Debug)]
struct OP1Var5 {
    imm16: TokenField_imm16,
}
impl OP1Var5 {
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
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::Y),
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
        if token_parser.TokenFieldbbb().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:95:1, end:95:4))"]
#[derive(Clone, Debug)]
struct OP1Var6 {
    imm8: TokenField_imm8,
}
impl OP1Var6 {
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
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("("),
            self.imm8.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::X),
            DisplayElement::Literal(")"),
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
        if token_parser.TokenFieldbbb().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:97:1, end:97:4))"]
#[derive(Clone, Debug)]
struct OP1Var7 {
    imm8: TokenField_imm8,
}
impl OP1Var7 {
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
            DisplayElement::Literal("("),
            self.imm8.display(),
            DisplayElement::Literal("),"),
            DisplayElement::Register(Register::Y),
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
        if token_parser.TokenFieldbbb().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableOP1 {
    Var0(OP1Var0),
    Var1(OP1Var1),
    Var2(OP1Var2),
    Var3(OP1Var3),
    Var4(OP1Var4),
    Var5(OP1Var5),
    Var6(OP1Var6),
    Var7(OP1Var7),
}
impl TableOP1 {
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
            OP1Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP1Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP1Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP1Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP1Var4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP1Var5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP1Var6::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP1Var7::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:100:1, end:100:4))"]
#[derive(Clone, Debug)]
struct OP2Var0 {
    imm8: TokenField_imm8,
}
impl OP2Var0 {
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
            [DisplayElement::Literal("#"), self.imm8.display()];
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
        if token_parser.TokenFieldbbb().disassembly() != 0i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:102:1, end:102:4))"]
#[derive(Clone, Debug)]
struct OP2Var1 {
    imm8: TokenField_imm8,
}
impl OP2Var1 {
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
        let extend: [DisplayElement; 1usize] = [self.imm8.display()];
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
        if token_parser.TokenFieldbbb().disassembly() != 1i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:103:1, end:103:4))"]
#[derive(Clone, Debug)]
struct OP2Var2 {}
impl OP2Var2 {
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
            [DisplayElement::Register(Register::A)];
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
        if token_parser.TokenFieldbbb().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:105:1, end:105:4))"]
#[derive(Clone, Debug)]
struct OP2Var3 {
    imm16: TokenField_imm16,
}
impl OP2Var3 {
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
        let extend: [DisplayElement; 1usize] = [self.imm16.display()];
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
        if token_parser.TokenFieldbbb().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:107:1, end:107:4))"]
#[derive(Clone, Debug)]
struct OP2Var4 {
    imm8: TokenField_imm8,
}
impl OP2Var4 {
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
            self.imm8.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::X),
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
        if token_parser.TokenFieldbbb().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:109:1, end:109:4))"]
#[derive(Clone, Debug)]
struct OP2Var5 {
    imm16: TokenField_imm16,
}
impl OP2Var5 {
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
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::X),
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
        if token_parser.TokenFieldbbb().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableOP2 {
    Var0(OP2Var0),
    Var1(OP2Var1),
    Var2(OP2Var2),
    Var3(OP2Var3),
    Var4(OP2Var4),
    Var5(OP2Var5),
}
impl TableOP2 {
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
            OP2Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2Var1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2Var2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2Var3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2Var4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2Var5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:112:1, end:112:6))"]
#[derive(Clone, Debug)]
struct OP2STVar0 {
    imm8: TokenField_imm8,
}
impl OP2STVar0 {
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
            self.imm8.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::Y),
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
        if token_parser.TokenFieldbbb().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:111:1, end:111:6))"]
#[derive(Clone, Debug)]
struct OP2STVar1 {
    OP2: TableOP2,
}
impl OP2STVar1 {
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
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[derive(Clone, Debug)]
enum TableOP2ST {
    Var0(OP2STVar0),
    Var1(OP2STVar1),
}
impl TableOP2ST {
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
            OP2STVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2STVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:115:1, end:115:6))"]
#[derive(Clone, Debug)]
struct OP2LDVar0 {
    imm8: TokenField_imm8,
}
impl OP2LDVar0 {
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
            self.imm8.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::Y),
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
        if token_parser.TokenFieldbbb().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:116:1, end:116:6))"]
#[derive(Clone, Debug)]
struct OP2LDVar1 {
    imm16: TokenField_imm16,
}
impl OP2LDVar1 {
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
            self.imm16.display(),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::Y),
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
        if token_parser.TokenFieldbbb().disassembly() != 7i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_1_len;
        tokens_current =
            &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:114:1, end:114:6))"]
#[derive(Clone, Debug)]
struct OP2LDVar2 {
    OP2: TableOP2,
}
impl OP2LDVar2 {
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
        self.OP2.display_extend(
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
        let mut block_0_len = 0u64 as u16;
        let OP2 = if let Some((len, table)) =
            TableOP2::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { OP2 }))
    }
}
#[derive(Clone, Debug)]
enum TableOP2LD {
    Var0(OP2LDVar0),
    Var1(OP2LDVar1),
    Var2(OP2LDVar2),
}
impl TableOP2LD {
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
            OP2LDVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2LDVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            OP2LDVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:119:1, end:119:6))"]
#[derive(Clone, Debug)]
struct ADDR8Var0 {
    imm8: TokenField_imm8,
}
impl ADDR8Var0 {
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
        let extend: [DisplayElement; 1usize] = [self.imm8.display()];
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
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableADDR8 {
    Var0(ADDR8Var0),
}
impl TableADDR8 {
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
            ADDR8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:120:1, end:120:7))"]
#[derive(Clone, Debug)]
struct ADDR16Var0 {
    imm16: TokenField_imm16,
}
impl ADDR16Var0 {
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
        let extend: [DisplayElement; 1usize] = [self.imm16.display()];
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
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableADDR16 {
    Var0(ADDR16Var0),
}
impl TableADDR16 {
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
            ADDR16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/6502/data/languages/6502.slaspec, start:121:1, end:121:6))"]
#[derive(Clone, Debug)]
struct ADDRIVar0 {
    imm16: TokenField_imm16,
}
impl ADDRIVar0 {
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
            DisplayElement::Literal("("),
            self.imm16.display(),
            DisplayElement::Literal(")"),
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let imm16 = token_parser.TokenFieldimm16();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableADDRI {
    Var0(ADDRIVar0),
}
impl TableADDRI {
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
            ADDRIVar0::parse(tokens_param, &mut context_current, inst_start)
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
