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
struct TokenField_op4_7(u8);
impl TokenField_op4_7 {
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
struct TokenField_op4_6(u8);
impl TokenField_op4_6 {
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
struct TokenField_nIndex(u8);
impl TokenField_nIndex {
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
struct TokenField_op0_0(u8);
impl TokenField_op0_0 {
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
struct TokenField_op16(u16);
impl TokenField_op16 {
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
struct TokenField_simm8(i8);
impl TokenField_simm8 {
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
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_op(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop4_7(&self) -> TokenField_op4_7 {
        let inner_value = self.read_u8::<true>(0, 4, 4).unwrap();
        TokenField_op4_7(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop4_6(&self) -> TokenField_op4_6 {
        let inner_value = self.read_u8::<true>(0, 4, 3).unwrap();
        TokenField_op4_6(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldnIndex(&self) -> TokenField_nIndex {
        let inner_value = self.read_u8::<true>(0, 1, 3).unwrap();
        TokenField_nIndex(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop0_0(&self) -> TokenField_op0_0 {
        let inner_value = self.read_u8::<true>(0, 0, 1).unwrap();
        TokenField_op0_0(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop16(&self) -> TokenField_op16 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_op16(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm8(&self) -> TokenField_imm8 {
        let inner_value = self.read_u8::<true>(0, 0, 8).unwrap();
        TokenField_imm8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldsimm8(&self) -> TokenField_simm8 {
        let inner_value = self.read_i8::<true>(0, 0, 8).unwrap();
        TokenField_simm8(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldrel(&self) -> TokenField_rel {
        let inner_value = self.read_i8::<true>(0, 0, 8).unwrap();
        TokenField_rel(i8::try_from(inner_value).unwrap())
    }
    fn TokenFieldimm16(&self) -> TokenField_imm16 {
        let inner_value = self.read_u16::<true>(0, 0, 16).unwrap();
        TokenField_imm16(u16::try_from(inner_value).unwrap())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    A,
    HIX,
    HI,
    X,
    PC,
    SP,
    PCH,
    PCL,
    SPH,
    SPL,
    CCR,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::HIX => write!(f, "HIX"),
            Self::HI => write!(f, "HI"),
            Self::X => write!(f, "X"),
            Self::PC => write!(f, "PC"),
            Self::SP => write!(f, "SP"),
            Self::PCH => write!(f, "PCH"),
            Self::PCL => write!(f, "PCL"),
            Self::SPH => write!(f, "SPH"),
            Self::SPL => write!(f, "SPL"),
            Self::CCR => write!(f, "CCR"),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:382:1, end:382:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar0 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl ADC_instructionVar0 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40665i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:391:1, end:391:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar1 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl ADC_instructionVar1 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40681i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:411:1, end:411:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar2 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl ADD_instructionVar2 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40667i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:422:1, end:422:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar3 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl ADD_instructionVar3 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40683i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:457:1, end:457:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar4 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl AND_instructionVar4 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40660i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:467:1, end:467:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar5 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl AND_instructionVar5 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40676i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:512:1, end:512:2))"]
#[derive(Clone, Debug)]
struct ASL_instructionVar6 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl ASL_instructionVar6 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40552i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:560:1, end:560:2))"]
#[derive(Clone, Debug)]
struct ASR_instructionVar7 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl ASR_instructionVar7 {
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
            [DisplayElement::Literal("ASR"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40551i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:671:1, end:671:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar8 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl BIT_instructionVar8 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40661i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:681:1, end:681:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar9 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl BIT_instructionVar9 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40677i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:839:1, end:839:2))"]
#[derive(Clone, Debug)]
struct CBEQ_instructionVar10 {
    oprx8_8_SP: Tableoprx8_8_SP,
    REL: TableREL,
}
impl CBEQ_instructionVar10 {
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
            DisplayElement::Literal("CBEQ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oprx8_8_SP.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c31 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40545i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c31(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { oprx8_8_SP, REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:900:1, end:900:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar11 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl CLR_instructionVar11 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40559i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:922:1, end:922:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar12 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl CMP_instructionVar12 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40657i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:934:1, end:934:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar13 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl CMP_instructionVar13 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40673i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:980:1, end:980:2))"]
#[derive(Clone, Debug)]
struct COM_instructionVar14 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl COM_instructionVar14 {
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
            [DisplayElement::Literal("COM"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40547i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1052:1, end:1052:2))"]
#[derive(Clone, Debug)]
struct CPX_instructionVar15 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl CPX_instructionVar15 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40659i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1064:1, end:1064:2))"]
#[derive(Clone, Debug)]
struct CPX_instructionVar16 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl CPX_instructionVar16 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40675i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1111:1, end:1111:2))"]
#[derive(Clone, Debug)]
struct DBNZ_instructionVar17 {
    oprx8_8_SP: Tableoprx8_8_SP,
    REL: TableREL,
}
impl DBNZ_instructionVar17 {
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
            DisplayElement::Literal("DBNZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oprx8_8_SP.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c42 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40555i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c42(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { oprx8_8_SP, REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1154:1, end:1154:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar18 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl DEC_instructionVar18 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40554i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1190:1, end:1190:2))"]
#[derive(Clone, Debug)]
struct EOR_instructionVar19 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl EOR_instructionVar19 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40664i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1201:1, end:1201:2))"]
#[derive(Clone, Debug)]
struct EOR_instructionVar20 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl EOR_instructionVar20 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40680i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1246:1, end:1246:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar21 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl INC_instructionVar21 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40556i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1302:1, end:1302:2))"]
#[derive(Clone, Debug)]
struct LDA_instructionVar22 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl LDA_instructionVar22 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40662i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1312:1, end:1312:2))"]
#[derive(Clone, Debug)]
struct LDA_instructionVar23 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl LDA_instructionVar23 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40678i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1402:1, end:1402:2))"]
#[derive(Clone, Debug)]
struct LDX_instructionVar24 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl LDX_instructionVar24 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40670i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1412:1, end:1412:2))"]
#[derive(Clone, Debug)]
struct LDX_instructionVar25 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl LDX_instructionVar25 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40686i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1462:1, end:1462:2))"]
#[derive(Clone, Debug)]
struct LSR_instructionVar26 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl LSR_instructionVar26 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40548i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1571:1, end:1571:2))"]
#[derive(Clone, Debug)]
struct NEG_instructionVar27 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl NEG_instructionVar27 {
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
            [DisplayElement::Literal("NEG"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40544i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1607:1, end:1607:2))"]
#[derive(Clone, Debug)]
struct ORA_instructionVar28 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl ORA_instructionVar28 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40666i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1617:1, end:1617:2))"]
#[derive(Clone, Debug)]
struct ORA_instructionVar29 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl ORA_instructionVar29 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40682i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1707:1, end:1707:2))"]
#[derive(Clone, Debug)]
struct ROL_instructionVar30 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl ROL_instructionVar30 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40553i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1759:1, end:1759:2))"]
#[derive(Clone, Debug)]
struct ROR_instructionVar31 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl ROR_instructionVar31 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40550i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1815:1, end:1815:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar32 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl SBC_instructionVar32 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40658i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1824:1, end:1824:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar33 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl SBC_instructionVar33 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40674i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1857:1, end:1857:2))"]
#[derive(Clone, Debug)]
struct STA_instructionVar34 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl STA_instructionVar34 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40663i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1867:1, end:1867:2))"]
#[derive(Clone, Debug)]
struct STA_instructionVar35 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl STA_instructionVar35 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40679i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1925:1, end:1925:2))"]
#[derive(Clone, Debug)]
struct STX_instructionVar36 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl STX_instructionVar36 {
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
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40671i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1935:1, end:1935:2))"]
#[derive(Clone, Debug)]
struct STX_instructionVar37 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl STX_instructionVar37 {
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
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40687i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1956:1, end:1956:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar38 {
    oprx16_8_SP: Tableoprx16_8_SP,
}
impl SUB_instructionVar38 {
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
            [DisplayElement::Literal("SUB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oprx16_8_SP.display_extend(
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
        let mut sub_pattern_c37 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40656i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c37(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_SP = if let Some((len, table)) = Tableoprx16_8_SP::parse(
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
        Some((pattern_len, Self { oprx16_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1967:1, end:1967:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar39 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl SUB_instructionVar39 {
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
            [DisplayElement::Literal("SUB"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40672i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2046:1, end:2046:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar40 {
    oprx8_8_SP: Tableoprx8_8_SP,
}
impl TST_instructionVar40 {
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
            [DisplayElement::Literal("TST"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.oprx8_8_SP.display_extend(
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
        let mut sub_pattern_c36 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2u64 as u16;
            let token_parser = <TokenParser<2usize>>::new(tokens)?;
            if token_parser.TokenFieldop16().disassembly() != 40557i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c36(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_SP = if let Some((len, table)) = Tableoprx8_8_SP::parse(
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
        Some((pattern_len, Self { oprx8_8_SP }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:433:1, end:433:2))"]
#[derive(Clone, Debug)]
struct AIS_instructionVar41 {
    iopr8is: Tableiopr8is,
}
impl AIS_instructionVar41 {
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
            [DisplayElement::Literal("AIS"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iopr8is.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 167i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr8is = if let Some((len, table)) = Tableiopr8is::parse(
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
        Some((pattern_len, Self { iopr8is }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:440:1, end:440:2))"]
#[derive(Clone, Debug)]
struct AIX_instructionVar42 {
    iopr8is: Tableiopr8is,
}
impl AIX_instructionVar42 {
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
            [DisplayElement::Literal("AIX"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iopr8is.display_extend(
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
        if token_parser.TokenFieldop().disassembly() != 175i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr8is = if let Some((len, table)) = Tableiopr8is::parse(
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
        Some((pattern_len, Self { iopr8is }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:477:1, end:477:2))"]
#[derive(Clone, Debug)]
struct ASLA_instructionVar43 {}
impl ASLA_instructionVar43 {
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
            [DisplayElement::Literal("ASLA")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:488:1, end:488:2))"]
#[derive(Clone, Debug)]
struct ASLX_instructionVar44 {}
impl ASLX_instructionVar44 {
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
            [DisplayElement::Literal("ASLX")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:525:1, end:525:2))"]
#[derive(Clone, Debug)]
struct ASRA_instructionVar45 {}
impl ASRA_instructionVar45 {
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
            [DisplayElement::Literal("ASRA")];
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
        if token_parser.TokenFieldop().disassembly() != 71i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:536:1, end:536:2))"]
#[derive(Clone, Debug)]
struct ASRX_instructionVar46 {}
impl ASRX_instructionVar46 {
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
            [DisplayElement::Literal("ASRX")];
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
        if token_parser.TokenFieldop().disassembly() != 87i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:573:1, end:573:2))"]
#[derive(Clone, Debug)]
struct BCC_instructionVar47 {
    REL: TableREL,
}
impl BCC_instructionVar47 {
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
        if token_parser.TokenFieldop().disassembly() != 36i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:587:1, end:587:2))"]
#[derive(Clone, Debug)]
struct BCS_instructionVar48 {
    REL: TableREL,
}
impl BCS_instructionVar48 {
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
        if token_parser.TokenFieldop().disassembly() != 37i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:594:1, end:594:2))"]
#[derive(Clone, Debug)]
struct BEQ_instructionVar49 {
    REL: TableREL,
}
impl BEQ_instructionVar49 {
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
        if token_parser.TokenFieldop().disassembly() != 39i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:601:1, end:601:2))"]
#[derive(Clone, Debug)]
struct BGE_instructionVar50 {
    REL: TableREL,
}
impl BGE_instructionVar50 {
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
            [DisplayElement::Literal("BGE"), DisplayElement::Literal(" ")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:615:1, end:615:2))"]
#[derive(Clone, Debug)]
struct BGT_instructionVar51 {
    REL: TableREL,
}
impl BGT_instructionVar51 {
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
            [DisplayElement::Literal("BGT"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 146i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:622:1, end:622:2))"]
#[derive(Clone, Debug)]
struct BHCC_instructionVar52 {
    REL: TableREL,
}
impl BHCC_instructionVar52 {
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
            DisplayElement::Literal("BHCC"),
            DisplayElement::Literal(" "),
        ];
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
        if token_parser.TokenFieldop().disassembly() != 40i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:629:1, end:629:2))"]
#[derive(Clone, Debug)]
struct BHCS_instructionVar53 {
    REL: TableREL,
}
impl BHCS_instructionVar53 {
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
            DisplayElement::Literal("BHCS"),
            DisplayElement::Literal(" "),
        ];
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
        if token_parser.TokenFieldop().disassembly() != 41i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:636:1, end:636:2))"]
#[derive(Clone, Debug)]
struct BHI_instructionVar54 {
    REL: TableREL,
}
impl BHI_instructionVar54 {
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
            [DisplayElement::Literal("BHI"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 34i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:645:1, end:645:2))"]
#[derive(Clone, Debug)]
struct BIH_instructionVar55 {
    REL: TableREL,
}
impl BIH_instructionVar55 {
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
            [DisplayElement::Literal("BIH"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 47i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:653:1, end:653:2))"]
#[derive(Clone, Debug)]
struct BIL_instructionVar56 {
    REL: TableREL,
}
impl BIL_instructionVar56 {
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
            [DisplayElement::Literal("BIL"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 46i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:691:1, end:691:2))"]
#[derive(Clone, Debug)]
struct BLE_instructionVar57 {
    REL: TableREL,
}
impl BLE_instructionVar57 {
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
            [DisplayElement::Literal("BLE"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 147i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:700:1, end:700:2))"]
#[derive(Clone, Debug)]
struct BLS_instructionVar58 {
    REL: TableREL,
}
impl BLS_instructionVar58 {
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
            [DisplayElement::Literal("BLS"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 35i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:707:1, end:707:2))"]
#[derive(Clone, Debug)]
struct BLT_instructionVar59 {
    REL: TableREL,
}
impl BLT_instructionVar59 {
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
            [DisplayElement::Literal("BLT"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 145i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:714:1, end:714:2))"]
#[derive(Clone, Debug)]
struct BMC_instructionVar60 {
    REL: TableREL,
}
impl BMC_instructionVar60 {
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
            [DisplayElement::Literal("BMC"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 44i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:721:1, end:721:2))"]
#[derive(Clone, Debug)]
struct BMI_instructionVar61 {
    REL: TableREL,
}
impl BMI_instructionVar61 {
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
        if token_parser.TokenFieldop().disassembly() != 43i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:728:1, end:728:2))"]
#[derive(Clone, Debug)]
struct BMS_instructionVar62 {
    REL: TableREL,
}
impl BMS_instructionVar62 {
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
            [DisplayElement::Literal("BMS"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 45i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:735:1, end:735:2))"]
#[derive(Clone, Debug)]
struct BNE_instructionVar63 {
    REL: TableREL,
}
impl BNE_instructionVar63 {
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
        if token_parser.TokenFieldop().disassembly() != 38i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:742:1, end:742:2))"]
#[derive(Clone, Debug)]
struct BPL_instructionVar64 {
    REL: TableREL,
}
impl BPL_instructionVar64 {
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
        if token_parser.TokenFieldop().disassembly() != 42i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:749:1, end:749:2))"]
#[derive(Clone, Debug)]
struct BRA_instructionVar65 {
    REL: TableREL,
}
impl BRA_instructionVar65 {
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
            [DisplayElement::Literal("BRA"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 32i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:766:1, end:766:2))"]
#[derive(Clone, Debug)]
struct BRN_instructionVar66 {
    REL: TableREL,
}
impl BRN_instructionVar66 {
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
            [DisplayElement::Literal("BRN"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 33i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:788:1, end:788:2))"]
#[derive(Clone, Debug)]
struct BSR_instructionVar67 {
    REL: TableREL,
}
impl BSR_instructionVar67 {
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
            [DisplayElement::Literal("BSR"), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 173i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:798:1, end:798:2))"]
#[derive(Clone, Debug)]
struct CBEQ_instructionVar68 {
    opr8a_8: Tableopr8a_8,
    REL: TableREL,
}
impl CBEQ_instructionVar68 {
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
            DisplayElement::Literal("CBEQ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c33 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 49i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c33(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { opr8a_8, REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:805:1, end:805:2))"]
#[derive(Clone, Debug)]
struct CBEQA_instructionVar69 {
    iopr8i: Tableiopr8i,
    REL: TableREL,
}
impl CBEQA_instructionVar69 {
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
            DisplayElement::Literal("CBEQA"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iopr8i.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 65i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr8i = if let Some((len, table)) = Tableiopr8i::parse(
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { iopr8i, REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:812:1, end:812:2))"]
#[derive(Clone, Debug)]
struct CBEQX_instructionVar70 {
    iopr8i: Tableiopr8i,
    REL: TableREL,
}
impl CBEQX_instructionVar70 {
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
            DisplayElement::Literal("CBEQX"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iopr8i.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop().disassembly() != 81i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr8i = if let Some((len, table)) = Tableiopr8i::parse(
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { iopr8i, REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:819:1, end:819:2))"]
#[derive(Clone, Debug)]
struct CBEQ_instructionVar71 {
    oprx8: Tableoprx8,
    REL: TableREL,
}
impl CBEQ_instructionVar71 {
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
            DisplayElement::Literal("CBEQ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.oprx8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::X),
            DisplayElement::Literal("+,,"),
            DisplayElement::Literal(" "),
        ];
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c33 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 97i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c33(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8 = if let Some((len, table)) =
            Tableoprx8::parse(tokens_current, &mut context_instance, inst_start)
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { oprx8, REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:829:1, end:829:2))"]
#[derive(Clone, Debug)]
struct CBEQ_instructionVar72 {
    REL: TableREL,
}
impl CBEQ_instructionVar72 {
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("CBEQ"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::X),
            DisplayElement::Literal("+,,"),
            DisplayElement::Literal(" "),
        ];
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 113i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:846:1, end:846:2))"]
#[derive(Clone, Debug)]
struct CLC_instructionVar73 {}
impl CLC_instructionVar73 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:853:1, end:853:2))"]
#[derive(Clone, Debug)]
struct CLI_instructionVar74 {}
impl CLI_instructionVar74 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:860:1, end:860:2))"]
#[derive(Clone, Debug)]
struct CLRA_instructionVar75 {}
impl CLRA_instructionVar75 {
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
            [DisplayElement::Literal("CLRA")];
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
        if token_parser.TokenFieldop().disassembly() != 79i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:870:1, end:870:2))"]
#[derive(Clone, Debug)]
struct CLRX_instructionVar76 {}
impl CLRX_instructionVar76 {
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
            [DisplayElement::Literal("CLRX")];
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
        if token_parser.TokenFieldop().disassembly() != 95i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:880:1, end:880:2))"]
#[derive(Clone, Debug)]
struct CLRH_instructionVar77 {}
impl CLRH_instructionVar77 {
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
            [DisplayElement::Literal("CLRH")];
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
        if token_parser.TokenFieldop().disassembly() != 140i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:946:1, end:946:2))"]
#[derive(Clone, Debug)]
struct COMA_instructionVar78 {}
impl COMA_instructionVar78 {
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
            [DisplayElement::Literal("COMA")];
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
        if token_parser.TokenFieldop().disassembly() != 67i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:957:1, end:957:2))"]
#[derive(Clone, Debug)]
struct COMX_instructionVar79 {}
impl COMX_instructionVar79 {
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
            [DisplayElement::Literal("COMX")];
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
        if token_parser.TokenFieldop().disassembly() != 83i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1004:1, end:1004:2))"]
#[derive(Clone, Debug)]
struct CPHX_instructionVar80 {
    iopr16i: Tableiopr16i,
}
impl CPHX_instructionVar80 {
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
            DisplayElement::Literal("CPHX"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iopr16i.display_extend(
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 101i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr16i = if let Some((len, table)) = Tableiopr16i::parse(
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
        Some((pattern_len, Self { iopr16i }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1016:1, end:1016:2))"]
#[derive(Clone, Debug)]
struct CPHX_instructionVar81 {
    opr8a_16: Tableopr8a_16,
}
impl CPHX_instructionVar81 {
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
            DisplayElement::Literal("CPHX"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_16.display_extend(
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
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 117i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_16 = if let Some((len, table)) = Tableopr8a_16::parse(
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
        Some((pattern_len, Self { opr8a_16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1076:1, end:1076:2))"]
#[derive(Clone, Debug)]
struct DAA_instructionVar82 {}
impl DAA_instructionVar82 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DAA")];
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
        if token_parser.TokenFieldop().disassembly() != 114i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1087:1, end:1087:2))"]
#[derive(Clone, Debug)]
struct DBNZA_instructionVar83 {
    REL: TableREL,
}
impl DBNZA_instructionVar83 {
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
            DisplayElement::Literal("DBNZA"),
            DisplayElement::Literal(" "),
        ];
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
        if token_parser.TokenFieldop().disassembly() != 75i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1095:1, end:1095:2))"]
#[derive(Clone, Debug)]
struct DBNZX_instructionVar84 {
    REL: TableREL,
}
impl DBNZX_instructionVar84 {
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
            DisplayElement::Literal("DBNZX"),
            DisplayElement::Literal(" "),
        ];
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
        if token_parser.TokenFieldop().disassembly() != 91i128 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1120:1, end:1120:2))"]
#[derive(Clone, Debug)]
struct DECA_instructionVar85 {}
impl DECA_instructionVar85 {
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
            [DisplayElement::Literal("DECA")];
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
        if token_parser.TokenFieldop().disassembly() != 74i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1131:1, end:1131:2))"]
#[derive(Clone, Debug)]
struct DECX_instructionVar86 {}
impl DECX_instructionVar86 {
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
            [DisplayElement::Literal("DECX")];
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
        if token_parser.TokenFieldop().disassembly() != 90i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1166:1, end:1166:2))"]
#[derive(Clone, Debug)]
struct DIV_instructionVar87 {}
impl DIV_instructionVar87 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("DIV")];
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
        if token_parser.TokenFieldop().disassembly() != 82i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1212:1, end:1212:2))"]
#[derive(Clone, Debug)]
struct INCA_instructionVar88 {}
impl INCA_instructionVar88 {
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
            [DisplayElement::Literal("INCA")];
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
        if token_parser.TokenFieldop().disassembly() != 76i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1223:1, end:1223:2))"]
#[derive(Clone, Debug)]
struct INCX_instructionVar89 {}
impl INCX_instructionVar89 {
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
            [DisplayElement::Literal("INCX")];
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
        if token_parser.TokenFieldop().disassembly() != 92i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1322:1, end:1322:2))"]
#[derive(Clone, Debug)]
struct LDHX_instructionVar90 {
    iopr16i: Tableiopr16i,
}
impl LDHX_instructionVar90 {
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
            DisplayElement::Literal("LDHX"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iopr16i.display_extend(
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
        let mut sub_pattern_c28 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 69i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c28(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr16i = if let Some((len, table)) = Tableiopr16i::parse(
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
        Some((pattern_len, Self { iopr16i }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1332:1, end:1332:2))"]
#[derive(Clone, Debug)]
struct LDHX_instructionVar91 {
    opr8a_16: Tableopr8a_16,
}
impl LDHX_instructionVar91 {
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
            DisplayElement::Literal("LDHX"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_16.display_extend(
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
        let mut sub_pattern_c33 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 85i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c33(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_16 = if let Some((len, table)) = Tableopr8a_16::parse(
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
        Some((pattern_len, Self { opr8a_16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1427:1, end:1427:2))"]
#[derive(Clone, Debug)]
struct LSRA_instructionVar92 {}
impl LSRA_instructionVar92 {
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
            [DisplayElement::Literal("LSRA")];
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
        if token_parser.TokenFieldop().disassembly() != 68i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1438:1, end:1438:2))"]
#[derive(Clone, Debug)]
struct LSRX_instructionVar93 {}
impl LSRX_instructionVar93 {
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
            [DisplayElement::Literal("LSRX")];
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
        if token_parser.TokenFieldop().disassembly() != 84i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1475:1, end:1475:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar94 {
    opr8a_8: Tableopr8a_8,
    op2_opr8a: Tableop2_opr8a,
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
        self.opr8a_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.op2_opr8a.display_extend(
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
        let mut sub_pattern_c28 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 78i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c28(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        let op2_opr8a = if let Some((len, table)) = Tableop2_opr8a::parse(
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
        Some((pattern_len, Self { opr8a_8, op2_opr8a }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1486:1, end:1486:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar95 {
    opr8a_8: Tableopr8a_8,
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
        self.opr8a_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::X),
            DisplayElement::Literal("+"),
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c27 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 94i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c27(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        Some((pattern_len, Self { opr8a_8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1498:1, end:1498:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar96 {
    iopr8i: Tableiopr8i,
    op2_opr8a: Tableop2_opr8a,
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
        self.iopr8i.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.op2_opr8a.display_extend(
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
        let mut sub_pattern_c28 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 110i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c28(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr8i = if let Some((len, table)) = Tableiopr8i::parse(
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
        let op2_opr8a = if let Some((len, table)) = Tableop2_opr8a::parse(
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
        Some((pattern_len, Self { iopr8i, op2_opr8a }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1509:1, end:1509:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar97 {
    op2_opr8a: Tableop2_opr8a,
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
        let extend: [DisplayElement; 6usize] = [
            DisplayElement::Literal("MOV"),
            DisplayElement::Literal(" "),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::X),
            DisplayElement::Literal("+,"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.op2_opr8a.display_extend(
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 126i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let op2_opr8a = if let Some((len, table)) = Tableop2_opr8a::parse(
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
        Some((pattern_len, Self { op2_opr8a }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1521:1, end:1521:2))"]
#[derive(Clone, Debug)]
struct MUL_instructionVar98 {}
impl MUL_instructionVar98 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("MUL")];
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
        if token_parser.TokenFieldop().disassembly() != 66i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1534:1, end:1534:2))"]
#[derive(Clone, Debug)]
struct NEGA_instructionVar99 {}
impl NEGA_instructionVar99 {
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
            [DisplayElement::Literal("NEGA")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1546:1, end:1546:2))"]
#[derive(Clone, Debug)]
struct NEGX_instructionVar100 {}
impl NEGX_instructionVar100 {
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
            [DisplayElement::Literal("NEGX")];
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
        if token_parser.TokenFieldop().disassembly() != 80i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1584:1, end:1584:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar101 {}
impl NOP_instructionVar101 {
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
        if token_parser.TokenFieldop().disassembly() != 157i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1590:1, end:1590:2))"]
#[derive(Clone, Debug)]
struct NSA_instructionVar102 {}
impl NSA_instructionVar102 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NSA")];
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
        if token_parser.TokenFieldop().disassembly() != 98i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1627:1, end:1627:2))"]
#[derive(Clone, Debug)]
struct PSHA_instructionVar103 {}
impl PSHA_instructionVar103 {
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
            [DisplayElement::Literal("PSHA")];
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
        if token_parser.TokenFieldop().disassembly() != 135i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1634:1, end:1634:2))"]
#[derive(Clone, Debug)]
struct PSHH_instructionVar104 {}
impl PSHH_instructionVar104 {
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
            [DisplayElement::Literal("PSHH")];
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
        if token_parser.TokenFieldop().disassembly() != 139i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1641:1, end:1641:2))"]
#[derive(Clone, Debug)]
struct PSHX_instructionVar105 {}
impl PSHX_instructionVar105 {
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
            [DisplayElement::Literal("PSHX")];
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
        if token_parser.TokenFieldop().disassembly() != 137i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1648:1, end:1648:2))"]
#[derive(Clone, Debug)]
struct PULA_instructionVar106 {}
impl PULA_instructionVar106 {
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
            [DisplayElement::Literal("PULA")];
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
        if token_parser.TokenFieldop().disassembly() != 134i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1655:1, end:1655:2))"]
#[derive(Clone, Debug)]
struct PULH_instructionVar107 {}
impl PULH_instructionVar107 {
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
            [DisplayElement::Literal("PULH")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1662:1, end:1662:2))"]
#[derive(Clone, Debug)]
struct PULX_instructionVar108 {}
impl PULX_instructionVar108 {
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
            [DisplayElement::Literal("PULX")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1669:1, end:1669:2))"]
#[derive(Clone, Debug)]
struct ROLA_instructionVar109 {}
impl ROLA_instructionVar109 {
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
            [DisplayElement::Literal("ROLA")];
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
        if token_parser.TokenFieldop().disassembly() != 73i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1681:1, end:1681:2))"]
#[derive(Clone, Debug)]
struct ROLX_instructionVar110 {}
impl ROLX_instructionVar110 {
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
            [DisplayElement::Literal("ROLX")];
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
        if token_parser.TokenFieldop().disassembly() != 89i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1721:1, end:1721:2))"]
#[derive(Clone, Debug)]
struct RORA_instructionVar111 {}
impl RORA_instructionVar111 {
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
            [DisplayElement::Literal("RORA")];
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
        if token_parser.TokenFieldop().disassembly() != 70i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1733:1, end:1733:2))"]
#[derive(Clone, Debug)]
struct RORX_instructionVar112 {}
impl RORX_instructionVar112 {
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
            [DisplayElement::Literal("RORX")];
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
        if token_parser.TokenFieldop().disassembly() != 86i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1773:1, end:1773:2))"]
#[derive(Clone, Debug)]
struct RSP_instructionVar113 {}
impl RSP_instructionVar113 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("RSP")];
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
        if token_parser.TokenFieldop().disassembly() != 156i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1780:1, end:1780:2))"]
#[derive(Clone, Debug)]
struct RTI_instructionVar114 {}
impl RTI_instructionVar114 {
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
        if token_parser.TokenFieldop().disassembly() != 128i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1796:1, end:1796:2))"]
#[derive(Clone, Debug)]
struct RTS_instructionVar115 {}
impl RTS_instructionVar115 {
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
        if token_parser.TokenFieldop().disassembly() != 129i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1833:1, end:1833:2))"]
#[derive(Clone, Debug)]
struct SEC_instructionVar116 {}
impl SEC_instructionVar116 {
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
        if token_parser.TokenFieldop().disassembly() != 153i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1840:1, end:1840:2))"]
#[derive(Clone, Debug)]
struct SEI_instructionVar117 {}
impl SEI_instructionVar117 {
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
        if token_parser.TokenFieldop().disassembly() != 155i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1877:1, end:1877:2))"]
#[derive(Clone, Debug)]
struct STHX_instructionVar118 {
    opr8a_16: Tableopr8a_16,
}
impl STHX_instructionVar118 {
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
            DisplayElement::Literal("STHX"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_16.display_extend(
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
        let mut sub_pattern_c33 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1u64 as u16;
            let token_parser = <TokenParser<1usize>>::new(tokens)?;
            if token_parser.TokenFieldop().disassembly() != 53i128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) =
            sub_pattern_c33(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_16 = if let Some((len, table)) = Tableopr8a_16::parse(
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
        Some((pattern_len, Self { opr8a_16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1907:1, end:1907:2))"]
#[derive(Clone, Debug)]
struct STOP_instructionVar119 {}
impl STOP_instructionVar119 {
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
            [DisplayElement::Literal("STOP")];
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
        if token_parser.TokenFieldop().disassembly() != 142i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1978:1, end:1978:2))"]
#[derive(Clone, Debug)]
struct SWI_instructionVar120 {}
impl SWI_instructionVar120 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("SWI")];
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
        if token_parser.TokenFieldop().disassembly() != 131i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1997:1, end:1997:2))"]
#[derive(Clone, Debug)]
struct TAP_instructionVar121 {}
impl TAP_instructionVar121 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TAP")];
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
        if token_parser.TokenFieldop().disassembly() != 132i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2004:1, end:2004:2))"]
#[derive(Clone, Debug)]
struct TAX_instructionVar122 {}
impl TAX_instructionVar122 {
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
        if token_parser.TokenFieldop().disassembly() != 151i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2011:1, end:2011:2))"]
#[derive(Clone, Debug)]
struct TPA_instructionVar123 {}
impl TPA_instructionVar123 {
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("TPA")];
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
        if token_parser.TokenFieldop().disassembly() != 133i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2018:1, end:2018:2))"]
#[derive(Clone, Debug)]
struct TSTA_instructionVar124 {}
impl TSTA_instructionVar124 {
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
            [DisplayElement::Literal("TSTA")];
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
        if token_parser.TokenFieldop().disassembly() != 77i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2027:1, end:2027:2))"]
#[derive(Clone, Debug)]
struct TSTX_instructionVar125 {}
impl TSTX_instructionVar125 {
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
            [DisplayElement::Literal("TSTX")];
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
        if token_parser.TokenFieldop().disassembly() != 93i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2056:1, end:2056:2))"]
#[derive(Clone, Debug)]
struct TSX_instructionVar126 {}
impl TSX_instructionVar126 {
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
        if token_parser.TokenFieldop().disassembly() != 149i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2063:1, end:2063:2))"]
#[derive(Clone, Debug)]
struct TXA_instructionVar127 {}
impl TXA_instructionVar127 {
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
        if token_parser.TokenFieldop().disassembly() != 159i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2070:1, end:2070:2))"]
#[derive(Clone, Debug)]
struct TXS_instructionVar128 {}
impl TXS_instructionVar128 {
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
        if token_parser.TokenFieldop().disassembly() != 148i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2077:1, end:2077:2))"]
#[derive(Clone, Debug)]
struct WAIT_instructionVar129 {}
impl WAIT_instructionVar129 {
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
            [DisplayElement::Literal("WAIT")];
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
        if token_parser.TokenFieldop().disassembly() != 143i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:499:1, end:499:2))"]
#[derive(Clone, Debug)]
struct ASL_instructionVar130 {
    OP1: TableOP1,
}
impl ASL_instructionVar130 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 56i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 104i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 120i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:547:1, end:547:2))"]
#[derive(Clone, Debug)]
struct ASR_instructionVar131 {
    OP1: TableOP1,
}
impl ASR_instructionVar131 {
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
            [DisplayElement::Literal("ASR"), DisplayElement::Literal(" ")];
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 55i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 103i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 119i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:890:1, end:890:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar132 {
    OP1: TableOP1,
}
impl CLR_instructionVar132 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 63i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 111i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 127i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:968:1, end:968:2))"]
#[derive(Clone, Debug)]
struct COM_instructionVar133 {
    OP1: TableOP1,
}
impl COM_instructionVar133 {
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
            [DisplayElement::Literal("COM"), DisplayElement::Literal(" ")];
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 51i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 99i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 115i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1103:1, end:1103:2))"]
#[derive(Clone, Debug)]
struct DBNZ_instructionVar134 {
    OP1: TableOP1,
    REL: TableREL,
}
impl DBNZ_instructionVar134 {
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
            DisplayElement::Literal("DBNZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.OP1.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        let mut block_0_len = 0u64 as u16;
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 59i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 107i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 123i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
        Some((pattern_len, Self { OP1, REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1142:1, end:1142:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar135 {
    OP1: TableOP1,
}
impl DEC_instructionVar135 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 58i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 106i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 122i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1234:1, end:1234:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar136 {
    OP1: TableOP1,
}
impl INC_instructionVar136 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 60i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 108i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 124i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1265:1, end:1265:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar137 {
    ADDRI: TableADDRI,
}
impl JMP_instructionVar137 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 220i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 236i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 252i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDRI = if let Some((len, table)) =
            TableADDRI::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { ADDRI }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1282:1, end:1282:2))"]
#[derive(Clone, Debug)]
struct JSR_instructionVar138 {
    ADDRI: TableADDRI,
}
impl JSR_instructionVar138 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 221i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 237i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 253i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDRI = if let Some((len, table)) =
            TableADDRI::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { ADDRI }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1449:1, end:1449:2))"]
#[derive(Clone, Debug)]
struct LSR_instructionVar139 {
    OP1: TableOP1,
}
impl LSR_instructionVar139 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 52i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 100i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 116i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1558:1, end:1558:2))"]
#[derive(Clone, Debug)]
struct NEG_instructionVar140 {
    OP1: TableOP1,
}
impl NEG_instructionVar140 {
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
            [DisplayElement::Literal("NEG"), DisplayElement::Literal(" ")];
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 48i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 96i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 112i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1693:1, end:1693:2))"]
#[derive(Clone, Debug)]
struct ROL_instructionVar141 {
    OP1: TableOP1,
}
impl ROL_instructionVar141 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 57i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 105i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 121i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1745:1, end:1745:2))"]
#[derive(Clone, Debug)]
struct ROR_instructionVar142 {
    OP1: TableOP1,
}
impl ROR_instructionVar142 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 54i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 102i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 118i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:2036:1, end:2036:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar143 {
    OP1: TableOP1,
}
impl TST_instructionVar143 {
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
            [DisplayElement::Literal("TST"), DisplayElement::Literal(" ")];
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 61i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 109i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 125i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:373:1, end:373:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar144 {
    OP1: TableOP1,
}
impl ADC_instructionVar144 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 169i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 185i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 201i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 217i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 233i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 249i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:400:1, end:400:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar145 {
    OP1: TableOP1,
}
impl ADD_instructionVar145 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 171i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 187i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 203i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 219i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 235i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 251i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:447:1, end:447:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar146 {
    OP1: TableOP1,
}
impl AND_instructionVar146 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 164i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 180i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 196i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 212i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 228i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 244i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:580:1, end:580:2))"]
#[derive(Clone, Debug)]
struct BCLR_instructionVar147 {
    nIndex: TokenField_nIndex,
    NthBit: TableNthBit,
    opr8a_8: Tableopr8a_8,
}
impl BCLR_instructionVar147 {
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
            DisplayElement::Literal("BCLR"),
            DisplayElement::Literal(" "),
            self.nIndex.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_8.display_extend(
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
        if token_parser.TokenFieldop4_7().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldop0_0().disassembly() != 1i128 {
            return None;
        }
        let NthBit = if let Some((len, table)) = TableNthBit::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let nIndex = token_parser.TokenFieldnIndex();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        Some((
            pattern_len,
            Self {
                NthBit,
                opr8a_8,
                nIndex,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:661:1, end:661:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar148 {
    OP1: TableOP1,
}
impl BIT_instructionVar148 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 165i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 181i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 197i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 213i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 229i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 245i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:756:1, end:756:2))"]
#[derive(Clone, Debug)]
struct BRCLR_instructionVar149 {
    nIndex: TokenField_nIndex,
    NthBit: TableNthBit,
    opr8a_8: Tableopr8a_8,
    REL: TableREL,
}
impl BRCLR_instructionVar149 {
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
            DisplayElement::Literal("BRCLR"),
            DisplayElement::Literal(" "),
            self.nIndex.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop4_7().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop0_0().disassembly() != 1i128 {
            return None;
        }
        let NthBit = if let Some((len, table)) = TableNthBit::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let nIndex = token_parser.TokenFieldnIndex();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
                NthBit,
                opr8a_8,
                REL,
                nIndex,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:772:1, end:772:2))"]
#[derive(Clone, Debug)]
struct BRSET_instructionVar150 {
    nIndex: TokenField_nIndex,
    NthBit: TableNthBit,
    opr8a_8: Tableopr8a_8,
    REL: TableREL,
}
impl BRSET_instructionVar150 {
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
            DisplayElement::Literal("BRSET"),
            DisplayElement::Literal(" "),
            self.nIndex.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(","), DisplayElement::Literal(" ")];
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
        if token_parser.TokenFieldop4_7().disassembly() != 0i128 {
            return None;
        }
        if token_parser.TokenFieldop0_0().disassembly() != 0i128 {
            return None;
        }
        let NthBit = if let Some((len, table)) = TableNthBit::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let nIndex = token_parser.TokenFieldnIndex();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
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
                NthBit,
                opr8a_8,
                REL,
                nIndex,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:781:1, end:781:2))"]
#[derive(Clone, Debug)]
struct BSET_instructionVar151 {
    nIndex: TokenField_nIndex,
    NthBit: TableNthBit,
    opr8a_8: Tableopr8a_8,
}
impl BSET_instructionVar151 {
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
            DisplayElement::Literal("BSET"),
            DisplayElement::Literal(" "),
            self.nIndex.display(),
            DisplayElement::Literal(","),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.opr8a_8.display_extend(
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
        if token_parser.TokenFieldop4_7().disassembly() != 1i128 {
            return None;
        }
        if token_parser.TokenFieldop0_0().disassembly() != 0i128 {
            return None;
        }
        let NthBit = if let Some((len, table)) = TableNthBit::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let nIndex = token_parser.TokenFieldnIndex();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        Some((
            pattern_len,
            Self {
                NthBit,
                opr8a_8,
                nIndex,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:910:1, end:910:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar152 {
    OP1: TableOP1,
}
impl CMP_instructionVar152 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 161i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 177i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 193i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 209i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 225i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 241i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1040:1, end:1040:2))"]
#[derive(Clone, Debug)]
struct CPX_instructionVar153 {
    OP1: TableOP1,
}
impl CPX_instructionVar153 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 163i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 179i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 195i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 211i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 227i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 243i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1179:1, end:1179:2))"]
#[derive(Clone, Debug)]
struct EOR_instructionVar154 {
    OP1: TableOP1,
}
impl EOR_instructionVar154 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 168i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 184i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 200i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 216i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 232i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 248i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1258:1, end:1258:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar155 {
    ADDR: TableADDR,
}
impl JMP_instructionVar155 {
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
        self.ADDR.display_extend(
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 188i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDR = if let Some((len, table)) =
            TableADDR::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { ADDR }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1272:1, end:1272:2))"]
#[derive(Clone, Debug)]
struct JSR_instructionVar156 {
    ADDR: TableADDR,
}
impl JSR_instructionVar156 {
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
        self.ADDR.display_extend(
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 189i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 205i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDR = if let Some((len, table)) =
            TableADDR::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { ADDR }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1292:1, end:1292:2))"]
#[derive(Clone, Debug)]
struct LDA_instructionVar157 {
    OP1: TableOP1,
}
impl LDA_instructionVar157 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 166i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 182i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 198i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 214i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 230i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 246i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1392:1, end:1392:2))"]
#[derive(Clone, Debug)]
struct LDX_instructionVar158 {
    OP1: TableOP1,
}
impl LDX_instructionVar158 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 174i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 190i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 206i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 222i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 238i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1597:1, end:1597:2))"]
#[derive(Clone, Debug)]
struct ORA_instructionVar159 {
    OP1: TableOP1,
}
impl ORA_instructionVar159 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 170i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 186i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 202i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 218i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 234i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 250i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1806:1, end:1806:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar160 {
    OP1: TableOP1,
}
impl SBC_instructionVar160 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 162i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 178i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 194i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 210i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 226i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 242i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1847:1, end:1847:2))"]
#[derive(Clone, Debug)]
struct STA_instructionVar161 {
    OP1: TableOP1,
}
impl STA_instructionVar161 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 183i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 199i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 215i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 231i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 247i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1915:1, end:1915:2))"]
#[derive(Clone, Debug)]
struct STX_instructionVar162 {
    OP1: TableOP1,
}
impl STX_instructionVar162 {
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 191i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 207i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 223i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 239i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 255i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:1945:1, end:1945:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar163 {
    OP1: TableOP1,
}
impl SUB_instructionVar163 {
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
            [DisplayElement::Literal("SUB"), DisplayElement::Literal(" ")];
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
        let mut sub_pattern_c29 = |tokens: &[u8], context_param: &mut T| {
            let mut pattern_len = 0 as u16;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut T| {
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 160i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 176i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 192i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 208i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 224i128 {
                    return Some(((), (), u16::try_from(1u64).unwrap()));
                }
                let token_parser = <TokenParser<1usize>>::new(tokens_param)?;
                if token_parser.TokenFieldop().disassembly() == 240i128 {
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
            sub_pattern_c29(tokens_current, &mut context_instance)?;
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
    Var0(ADC_instructionVar0),
    Var1(ADC_instructionVar1),
    Var2(ADD_instructionVar2),
    Var3(ADD_instructionVar3),
    Var4(AND_instructionVar4),
    Var5(AND_instructionVar5),
    Var6(ASL_instructionVar6),
    Var7(ASR_instructionVar7),
    Var8(BIT_instructionVar8),
    Var9(BIT_instructionVar9),
    Var10(CBEQ_instructionVar10),
    Var11(CLR_instructionVar11),
    Var12(CMP_instructionVar12),
    Var13(CMP_instructionVar13),
    Var14(COM_instructionVar14),
    Var15(CPX_instructionVar15),
    Var16(CPX_instructionVar16),
    Var17(DBNZ_instructionVar17),
    Var18(DEC_instructionVar18),
    Var19(EOR_instructionVar19),
    Var20(EOR_instructionVar20),
    Var21(INC_instructionVar21),
    Var22(LDA_instructionVar22),
    Var23(LDA_instructionVar23),
    Var24(LDX_instructionVar24),
    Var25(LDX_instructionVar25),
    Var26(LSR_instructionVar26),
    Var27(NEG_instructionVar27),
    Var28(ORA_instructionVar28),
    Var29(ORA_instructionVar29),
    Var30(ROL_instructionVar30),
    Var31(ROR_instructionVar31),
    Var32(SBC_instructionVar32),
    Var33(SBC_instructionVar33),
    Var34(STA_instructionVar34),
    Var35(STA_instructionVar35),
    Var36(STX_instructionVar36),
    Var37(STX_instructionVar37),
    Var38(SUB_instructionVar38),
    Var39(SUB_instructionVar39),
    Var40(TST_instructionVar40),
    Var41(AIS_instructionVar41),
    Var42(AIX_instructionVar42),
    Var43(ASLA_instructionVar43),
    Var44(ASLX_instructionVar44),
    Var45(ASRA_instructionVar45),
    Var46(ASRX_instructionVar46),
    Var47(BCC_instructionVar47),
    Var48(BCS_instructionVar48),
    Var49(BEQ_instructionVar49),
    Var50(BGE_instructionVar50),
    Var51(BGT_instructionVar51),
    Var52(BHCC_instructionVar52),
    Var53(BHCS_instructionVar53),
    Var54(BHI_instructionVar54),
    Var55(BIH_instructionVar55),
    Var56(BIL_instructionVar56),
    Var57(BLE_instructionVar57),
    Var58(BLS_instructionVar58),
    Var59(BLT_instructionVar59),
    Var60(BMC_instructionVar60),
    Var61(BMI_instructionVar61),
    Var62(BMS_instructionVar62),
    Var63(BNE_instructionVar63),
    Var64(BPL_instructionVar64),
    Var65(BRA_instructionVar65),
    Var66(BRN_instructionVar66),
    Var67(BSR_instructionVar67),
    Var68(CBEQ_instructionVar68),
    Var69(CBEQA_instructionVar69),
    Var70(CBEQX_instructionVar70),
    Var71(CBEQ_instructionVar71),
    Var72(CBEQ_instructionVar72),
    Var73(CLC_instructionVar73),
    Var74(CLI_instructionVar74),
    Var75(CLRA_instructionVar75),
    Var76(CLRX_instructionVar76),
    Var77(CLRH_instructionVar77),
    Var78(COMA_instructionVar78),
    Var79(COMX_instructionVar79),
    Var80(CPHX_instructionVar80),
    Var81(CPHX_instructionVar81),
    Var82(DAA_instructionVar82),
    Var83(DBNZA_instructionVar83),
    Var84(DBNZX_instructionVar84),
    Var85(DECA_instructionVar85),
    Var86(DECX_instructionVar86),
    Var87(DIV_instructionVar87),
    Var88(INCA_instructionVar88),
    Var89(INCX_instructionVar89),
    Var90(LDHX_instructionVar90),
    Var91(LDHX_instructionVar91),
    Var92(LSRA_instructionVar92),
    Var93(LSRX_instructionVar93),
    Var94(MOV_instructionVar94),
    Var95(MOV_instructionVar95),
    Var96(MOV_instructionVar96),
    Var97(MOV_instructionVar97),
    Var98(MUL_instructionVar98),
    Var99(NEGA_instructionVar99),
    Var100(NEGX_instructionVar100),
    Var101(NOP_instructionVar101),
    Var102(NSA_instructionVar102),
    Var103(PSHA_instructionVar103),
    Var104(PSHH_instructionVar104),
    Var105(PSHX_instructionVar105),
    Var106(PULA_instructionVar106),
    Var107(PULH_instructionVar107),
    Var108(PULX_instructionVar108),
    Var109(ROLA_instructionVar109),
    Var110(ROLX_instructionVar110),
    Var111(RORA_instructionVar111),
    Var112(RORX_instructionVar112),
    Var113(RSP_instructionVar113),
    Var114(RTI_instructionVar114),
    Var115(RTS_instructionVar115),
    Var116(SEC_instructionVar116),
    Var117(SEI_instructionVar117),
    Var118(STHX_instructionVar118),
    Var119(STOP_instructionVar119),
    Var120(SWI_instructionVar120),
    Var121(TAP_instructionVar121),
    Var122(TAX_instructionVar122),
    Var123(TPA_instructionVar123),
    Var124(TSTA_instructionVar124),
    Var125(TSTX_instructionVar125),
    Var126(TSX_instructionVar126),
    Var127(TXA_instructionVar127),
    Var128(TXS_instructionVar128),
    Var129(WAIT_instructionVar129),
    Var130(ASL_instructionVar130),
    Var131(ASR_instructionVar131),
    Var132(CLR_instructionVar132),
    Var133(COM_instructionVar133),
    Var134(DBNZ_instructionVar134),
    Var135(DEC_instructionVar135),
    Var136(INC_instructionVar136),
    Var137(JMP_instructionVar137),
    Var138(JSR_instructionVar138),
    Var139(LSR_instructionVar139),
    Var140(NEG_instructionVar140),
    Var141(ROL_instructionVar141),
    Var142(ROR_instructionVar142),
    Var143(TST_instructionVar143),
    Var144(ADC_instructionVar144),
    Var145(ADD_instructionVar145),
    Var146(AND_instructionVar146),
    Var147(BCLR_instructionVar147),
    Var148(BIT_instructionVar148),
    Var149(BRCLR_instructionVar149),
    Var150(BRSET_instructionVar150),
    Var151(BSET_instructionVar151),
    Var152(CMP_instructionVar152),
    Var153(CPX_instructionVar153),
    Var154(EOR_instructionVar154),
    Var155(JMP_instructionVar155),
    Var156(JSR_instructionVar156),
    Var157(LDA_instructionVar157),
    Var158(LDX_instructionVar158),
    Var159(ORA_instructionVar159),
    Var160(SBC_instructionVar160),
    Var161(STA_instructionVar161),
    Var162(STX_instructionVar162),
    Var163(SUB_instructionVar163),
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
        if let Some((inst_len, parsed)) = ADC_instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        if let Some((inst_len, parsed)) = ASL_instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var6(parsed)));
        }
        if let Some((inst_len, parsed)) = ASR_instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var7(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var8(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var9(parsed)));
        }
        if let Some((inst_len, parsed)) = CBEQ_instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var10(parsed)));
        }
        if let Some((inst_len, parsed)) = CLR_instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var11(parsed)));
        }
        if let Some((inst_len, parsed)) = CMP_instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var12(parsed)));
        }
        if let Some((inst_len, parsed)) = CMP_instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var13(parsed)));
        }
        if let Some((inst_len, parsed)) = COM_instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var14(parsed)));
        }
        if let Some((inst_len, parsed)) = CPX_instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var15(parsed)));
        }
        if let Some((inst_len, parsed)) = CPX_instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var16(parsed)));
        }
        if let Some((inst_len, parsed)) = DBNZ_instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var17(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var18(parsed)));
        }
        if let Some((inst_len, parsed)) = EOR_instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var19(parsed)));
        }
        if let Some((inst_len, parsed)) = EOR_instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var20(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var21(parsed)));
        }
        if let Some((inst_len, parsed)) = LDA_instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var22(parsed)));
        }
        if let Some((inst_len, parsed)) = LDA_instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var23(parsed)));
        }
        if let Some((inst_len, parsed)) = LDX_instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var24(parsed)));
        }
        if let Some((inst_len, parsed)) = LDX_instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var25(parsed)));
        }
        if let Some((inst_len, parsed)) = LSR_instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var26(parsed)));
        }
        if let Some((inst_len, parsed)) = NEG_instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var27(parsed)));
        }
        if let Some((inst_len, parsed)) = ORA_instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var28(parsed)));
        }
        if let Some((inst_len, parsed)) = ORA_instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var29(parsed)));
        }
        if let Some((inst_len, parsed)) = ROL_instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var30(parsed)));
        }
        if let Some((inst_len, parsed)) = ROR_instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var31(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var32(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var33(parsed)));
        }
        if let Some((inst_len, parsed)) = STA_instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var34(parsed)));
        }
        if let Some((inst_len, parsed)) = STA_instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var35(parsed)));
        }
        if let Some((inst_len, parsed)) = STX_instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var36(parsed)));
        }
        if let Some((inst_len, parsed)) = STX_instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var37(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var38(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var39(parsed)));
        }
        if let Some((inst_len, parsed)) = TST_instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var40(parsed)));
        }
        if let Some((inst_len, parsed)) = AIS_instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var41(parsed)));
        }
        if let Some((inst_len, parsed)) = AIX_instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var42(parsed)));
        }
        if let Some((inst_len, parsed)) = ASLA_instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var43(parsed)));
        }
        if let Some((inst_len, parsed)) = ASLX_instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var44(parsed)));
        }
        if let Some((inst_len, parsed)) = ASRA_instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var45(parsed)));
        }
        if let Some((inst_len, parsed)) = ASRX_instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var46(parsed)));
        }
        if let Some((inst_len, parsed)) = BCC_instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var47(parsed)));
        }
        if let Some((inst_len, parsed)) = BCS_instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var48(parsed)));
        }
        if let Some((inst_len, parsed)) = BEQ_instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var49(parsed)));
        }
        if let Some((inst_len, parsed)) = BGE_instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var50(parsed)));
        }
        if let Some((inst_len, parsed)) = BGT_instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var51(parsed)));
        }
        if let Some((inst_len, parsed)) = BHCC_instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var52(parsed)));
        }
        if let Some((inst_len, parsed)) = BHCS_instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var53(parsed)));
        }
        if let Some((inst_len, parsed)) = BHI_instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var54(parsed)));
        }
        if let Some((inst_len, parsed)) = BIH_instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var55(parsed)));
        }
        if let Some((inst_len, parsed)) = BIL_instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var56(parsed)));
        }
        if let Some((inst_len, parsed)) = BLE_instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var57(parsed)));
        }
        if let Some((inst_len, parsed)) = BLS_instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var58(parsed)));
        }
        if let Some((inst_len, parsed)) = BLT_instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var59(parsed)));
        }
        if let Some((inst_len, parsed)) = BMC_instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var60(parsed)));
        }
        if let Some((inst_len, parsed)) = BMI_instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var61(parsed)));
        }
        if let Some((inst_len, parsed)) = BMS_instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var62(parsed)));
        }
        if let Some((inst_len, parsed)) = BNE_instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var63(parsed)));
        }
        if let Some((inst_len, parsed)) = BPL_instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var64(parsed)));
        }
        if let Some((inst_len, parsed)) = BRA_instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var65(parsed)));
        }
        if let Some((inst_len, parsed)) = BRN_instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var66(parsed)));
        }
        if let Some((inst_len, parsed)) = BSR_instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var67(parsed)));
        }
        if let Some((inst_len, parsed)) = CBEQ_instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var68(parsed)));
        }
        if let Some((inst_len, parsed)) = CBEQA_instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var69(parsed)));
        }
        if let Some((inst_len, parsed)) = CBEQX_instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var70(parsed)));
        }
        if let Some((inst_len, parsed)) = CBEQ_instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var71(parsed)));
        }
        if let Some((inst_len, parsed)) = CBEQ_instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var72(parsed)));
        }
        if let Some((inst_len, parsed)) = CLC_instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var73(parsed)));
        }
        if let Some((inst_len, parsed)) = CLI_instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var74(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRA_instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var75(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRX_instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var76(parsed)));
        }
        if let Some((inst_len, parsed)) = CLRH_instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var77(parsed)));
        }
        if let Some((inst_len, parsed)) = COMA_instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var78(parsed)));
        }
        if let Some((inst_len, parsed)) = COMX_instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var79(parsed)));
        }
        if let Some((inst_len, parsed)) = CPHX_instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var80(parsed)));
        }
        if let Some((inst_len, parsed)) = CPHX_instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var81(parsed)));
        }
        if let Some((inst_len, parsed)) = DAA_instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var82(parsed)));
        }
        if let Some((inst_len, parsed)) = DBNZA_instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var83(parsed)));
        }
        if let Some((inst_len, parsed)) = DBNZX_instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var84(parsed)));
        }
        if let Some((inst_len, parsed)) = DECA_instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var85(parsed)));
        }
        if let Some((inst_len, parsed)) = DECX_instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var86(parsed)));
        }
        if let Some((inst_len, parsed)) = DIV_instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var87(parsed)));
        }
        if let Some((inst_len, parsed)) = INCA_instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var88(parsed)));
        }
        if let Some((inst_len, parsed)) = INCX_instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var89(parsed)));
        }
        if let Some((inst_len, parsed)) = LDHX_instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var90(parsed)));
        }
        if let Some((inst_len, parsed)) = LDHX_instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var91(parsed)));
        }
        if let Some((inst_len, parsed)) = LSRA_instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var92(parsed)));
        }
        if let Some((inst_len, parsed)) = LSRX_instructionVar93::parse(
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
        if let Some((inst_len, parsed)) = MUL_instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var98(parsed)));
        }
        if let Some((inst_len, parsed)) = NEGA_instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var99(parsed)));
        }
        if let Some((inst_len, parsed)) = NEGX_instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var100(parsed)));
        }
        if let Some((inst_len, parsed)) = NOP_instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var101(parsed)));
        }
        if let Some((inst_len, parsed)) = NSA_instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var102(parsed)));
        }
        if let Some((inst_len, parsed)) = PSHA_instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var103(parsed)));
        }
        if let Some((inst_len, parsed)) = PSHH_instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var104(parsed)));
        }
        if let Some((inst_len, parsed)) = PSHX_instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var105(parsed)));
        }
        if let Some((inst_len, parsed)) = PULA_instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var106(parsed)));
        }
        if let Some((inst_len, parsed)) = PULH_instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var107(parsed)));
        }
        if let Some((inst_len, parsed)) = PULX_instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var108(parsed)));
        }
        if let Some((inst_len, parsed)) = ROLA_instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var109(parsed)));
        }
        if let Some((inst_len, parsed)) = ROLX_instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var110(parsed)));
        }
        if let Some((inst_len, parsed)) = RORA_instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var111(parsed)));
        }
        if let Some((inst_len, parsed)) = RORX_instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var112(parsed)));
        }
        if let Some((inst_len, parsed)) = RSP_instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var113(parsed)));
        }
        if let Some((inst_len, parsed)) = RTI_instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var114(parsed)));
        }
        if let Some((inst_len, parsed)) = RTS_instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var115(parsed)));
        }
        if let Some((inst_len, parsed)) = SEC_instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var116(parsed)));
        }
        if let Some((inst_len, parsed)) = SEI_instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var117(parsed)));
        }
        if let Some((inst_len, parsed)) = STHX_instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var118(parsed)));
        }
        if let Some((inst_len, parsed)) = STOP_instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var119(parsed)));
        }
        if let Some((inst_len, parsed)) = SWI_instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var120(parsed)));
        }
        if let Some((inst_len, parsed)) = TAP_instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var121(parsed)));
        }
        if let Some((inst_len, parsed)) = TAX_instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var122(parsed)));
        }
        if let Some((inst_len, parsed)) = TPA_instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var123(parsed)));
        }
        if let Some((inst_len, parsed)) = TSTA_instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var124(parsed)));
        }
        if let Some((inst_len, parsed)) = TSTX_instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var125(parsed)));
        }
        if let Some((inst_len, parsed)) = TSX_instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var126(parsed)));
        }
        if let Some((inst_len, parsed)) = TXA_instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var127(parsed)));
        }
        if let Some((inst_len, parsed)) = TXS_instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var128(parsed)));
        }
        if let Some((inst_len, parsed)) = WAIT_instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var129(parsed)));
        }
        if let Some((inst_len, parsed)) = ASL_instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var130(parsed)));
        }
        if let Some((inst_len, parsed)) = ASR_instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var131(parsed)));
        }
        if let Some((inst_len, parsed)) = CLR_instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var132(parsed)));
        }
        if let Some((inst_len, parsed)) = COM_instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var133(parsed)));
        }
        if let Some((inst_len, parsed)) = DBNZ_instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var134(parsed)));
        }
        if let Some((inst_len, parsed)) = DEC_instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var135(parsed)));
        }
        if let Some((inst_len, parsed)) = INC_instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var136(parsed)));
        }
        if let Some((inst_len, parsed)) = JMP_instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var137(parsed)));
        }
        if let Some((inst_len, parsed)) = JSR_instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var138(parsed)));
        }
        if let Some((inst_len, parsed)) = LSR_instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var139(parsed)));
        }
        if let Some((inst_len, parsed)) = NEG_instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var140(parsed)));
        }
        if let Some((inst_len, parsed)) = ROL_instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var141(parsed)));
        }
        if let Some((inst_len, parsed)) = ROR_instructionVar142::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var142(parsed)));
        }
        if let Some((inst_len, parsed)) = TST_instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var143(parsed)));
        }
        if let Some((inst_len, parsed)) = ADC_instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var144(parsed)));
        }
        if let Some((inst_len, parsed)) = ADD_instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var145(parsed)));
        }
        if let Some((inst_len, parsed)) = AND_instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var146(parsed)));
        }
        if let Some((inst_len, parsed)) = BCLR_instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var147(parsed)));
        }
        if let Some((inst_len, parsed)) = BIT_instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var148(parsed)));
        }
        if let Some((inst_len, parsed)) = BRCLR_instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var149(parsed)));
        }
        if let Some((inst_len, parsed)) = BRSET_instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var150(parsed)));
        }
        if let Some((inst_len, parsed)) = BSET_instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var151(parsed)));
        }
        if let Some((inst_len, parsed)) = CMP_instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var152(parsed)));
        }
        if let Some((inst_len, parsed)) = CPX_instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var153(parsed)));
        }
        if let Some((inst_len, parsed)) = EOR_instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var154(parsed)));
        }
        if let Some((inst_len, parsed)) = JMP_instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var155(parsed)));
        }
        if let Some((inst_len, parsed)) = JSR_instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var156(parsed)));
        }
        if let Some((inst_len, parsed)) = LDA_instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var157(parsed)));
        }
        if let Some((inst_len, parsed)) = LDX_instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var158(parsed)));
        }
        if let Some((inst_len, parsed)) = ORA_instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var159(parsed)));
        }
        if let Some((inst_len, parsed)) = SBC_instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var160(parsed)));
        }
        if let Some((inst_len, parsed)) = STA_instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var161(parsed)));
        }
        if let Some((inst_len, parsed)) = STX_instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var162(parsed)));
        }
        if let Some((inst_len, parsed)) = SUB_instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
        ) {
            *context_param = context_current;
            return Some((inst_len, Self::Var163(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:95:1, end:95:8))"]
#[derive(Clone, Debug)]
struct opr8a_8Var0 {
    imm8: TokenField_imm8,
}
impl opr8a_8Var0 {
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
enum Tableopr8a_8 {
    Var0(opr8a_8Var0),
}
impl Tableopr8a_8 {
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
            opr8a_8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:96:1, end:96:9))"]
#[derive(Clone, Debug)]
struct opr16a_8Var0 {
    imm16: TokenField_imm16,
}
impl opr16a_8Var0 {
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
enum Tableopr16a_8 {
    Var0(opr16a_8Var0),
}
impl Tableopr16a_8 {
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
            opr16a_8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:97:1, end:97:7))"]
#[derive(Clone, Debug)]
struct iopr8iVar0 {
    imm8: TokenField_imm8,
}
impl iopr8iVar0 {
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
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableiopr8i {
    Var0(iopr8iVar0),
}
impl Tableiopr8i {
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
            iopr8iVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:101:1, end:101:9))"]
#[derive(Clone, Debug)]
struct opr8a_16Var0 {
    imm8: TokenField_imm8,
}
impl opr8a_16Var0 {
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
enum Tableopr8a_16 {
    Var0(opr8a_16Var0),
}
impl Tableopr8a_16 {
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
            opr8a_16Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:102:1, end:102:8))"]
#[derive(Clone, Debug)]
struct iopr8isVar0 {
    simm8: TokenField_simm8,
}
impl iopr8isVar0 {
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
            [DisplayElement::Literal("#"), self.simm8.display()];
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
        let simm8 = token_parser.TokenFieldsimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableiopr8is {
    Var0(iopr8isVar0),
}
impl Tableiopr8is {
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
            iopr8isVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:103:1, end:103:8))"]
#[derive(Clone, Debug)]
struct iopr16iVar0 {
    imm16: TokenField_imm16,
}
impl iopr16iVar0 {
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
            [DisplayElement::Literal("#"), self.imm16.display()];
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
enum Tableiopr16i {
    Var0(iopr16iVar0),
}
impl Tableiopr16i {
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
            iopr16iVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:104:1, end:104:6))"]
#[derive(Clone, Debug)]
struct oprx8Var0 {
    imm8: TokenField_imm8,
}
impl oprx8Var0 {
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
enum Tableoprx8 {
    Var0(oprx8Var0),
}
impl Tableoprx8 {
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
            oprx8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:105:1, end:105:11))"]
#[derive(Clone, Debug)]
struct oprx8_8_SPVar0 {
    imm8: TokenField_imm8,
}
impl oprx8_8_SPVar0 {
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
            DisplayElement::Register(Register::SP),
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
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableoprx8_8_SP {
    Var0(oprx8_8_SPVar0),
}
impl Tableoprx8_8_SP {
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
        if let Some((inst_len, parsed)) = oprx8_8_SPVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:106:1, end:106:12))"]
#[derive(Clone, Debug)]
struct oprx16_8_SPVar0 {
    imm16: TokenField_imm16,
}
impl oprx16_8_SPVar0 {
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
            DisplayElement::Register(Register::SP),
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
enum Tableoprx16_8_SP {
    Var0(oprx16_8_SPVar0),
}
impl Tableoprx16_8_SP {
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
        if let Some((inst_len, parsed)) = oprx16_8_SPVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:122:1, end:122:10))"]
#[derive(Clone, Debug)]
struct oprx8_8_XVar0 {
    imm8: TokenField_imm8,
}
impl oprx8_8_XVar0 {
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
        let imm8 = token_parser.TokenFieldimm8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum Tableoprx8_8_X {
    Var0(oprx8_8_XVar0),
}
impl Tableoprx8_8_X {
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
            oprx8_8_XVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:123:1, end:123:11))"]
#[derive(Clone, Debug)]
struct oprx16_8_XVar0 {
    imm16: TokenField_imm16,
}
impl oprx16_8_XVar0 {
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
enum Tableoprx16_8_X {
    Var0(oprx16_8_XVar0),
}
impl Tableoprx16_8_X {
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
        if let Some((inst_len, parsed)) = oprx16_8_XVar0::parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:124:1, end:124:8))"]
#[derive(Clone, Debug)]
struct comma_XVar0 {}
impl comma_XVar0 {
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
        let mut block_0_len = 0u64 as u16;
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablecomma_X {
    Var0(comma_XVar0),
}
impl Tablecomma_X {
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
            comma_XVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:135:1, end:135:4))"]
#[derive(Clone, Debug)]
struct OP1Var0 {
    iopr8i: Tableiopr8i,
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
        self.iopr8i.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 2i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let iopr8i = if let Some((len, table)) = Tableiopr8i::parse(
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
        Some((pattern_len, Self { iopr8i }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:136:1, end:136:4))"]
#[derive(Clone, Debug)]
struct OP1Var1 {
    opr8a_8: Tableopr8a_8,
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
        self.opr8a_8.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        Some((pattern_len, Self { opr8a_8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:137:1, end:137:4))"]
#[derive(Clone, Debug)]
struct OP1Var2 {
    opr16a_8: Tableopr16a_8,
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
        self.opr16a_8.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr16a_8 = if let Some((len, table)) = Tableopr16a_8::parse(
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
        Some((pattern_len, Self { opr16a_8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:140:1, end:140:4))"]
#[derive(Clone, Debug)]
struct OP1Var3 {
    oprx16_8_X: Tableoprx16_8_X,
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
        self.oprx16_8_X.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_X = if let Some((len, table)) = Tableoprx16_8_X::parse(
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
        Some((pattern_len, Self { oprx16_8_X }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:143:1, end:143:4))"]
#[derive(Clone, Debug)]
struct OP1Var4 {
    oprx8_8_X: Tableoprx8_8_X,
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
        self.oprx8_8_X.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_X = if let Some((len, table)) = Tableoprx8_8_X::parse(
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
        Some((pattern_len, Self { oprx8_8_X }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:144:1, end:144:4))"]
#[derive(Clone, Debug)]
struct OP1Var5 {
    comma_X: Tablecomma_X,
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
        self.comma_X.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 7i128 {
            return None;
        }
        let comma_X = if let Some((len, table)) = Tablecomma_X::parse(
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
        Some((pattern_len, Self { comma_X }))
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
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:147:1, end:147:10))"]
#[derive(Clone, Debug)]
struct op2_opr8aVar0 {
    imm8: TokenField_imm8,
}
impl op2_opr8aVar0 {
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
enum Tableop2_opr8a {
    Var0(op2_opr8aVar0),
}
impl Tableop2_opr8a {
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
            op2_opr8aVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:151:1, end:151:5))"]
#[derive(Clone, Debug)]
struct ADDRVar0 {
    opr8a_8: Tableopr8a_8,
}
impl ADDRVar0 {
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
        self.opr8a_8.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 3i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr8a_8 = if let Some((len, table)) = Tableopr8a_8::parse(
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
        Some((pattern_len, Self { opr8a_8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:152:1, end:152:5))"]
#[derive(Clone, Debug)]
struct ADDRVar1 {
    opr16a_8: Tableopr16a_8,
}
impl ADDRVar1 {
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
        self.opr16a_8.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 4i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let opr16a_8 = if let Some((len, table)) = Tableopr16a_8::parse(
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
        Some((pattern_len, Self { opr16a_8 }))
    }
}
#[derive(Clone, Debug)]
enum TableADDR {
    Var0(ADDRVar0),
    Var1(ADDRVar1),
}
impl TableADDR {
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
            ADDRVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ADDRVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:155:1, end:155:6))"]
#[derive(Clone, Debug)]
struct ADDRIVar0 {
    oprx16_8_X: Tableoprx16_8_X,
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
        self.oprx16_8_X.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 5i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx16_8_X = if let Some((len, table)) = Tableoprx16_8_X::parse(
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
        Some((pattern_len, Self { oprx16_8_X }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:158:1, end:158:6))"]
#[derive(Clone, Debug)]
struct ADDRIVar1 {
    oprx8_8_X: Tableoprx8_8_X,
}
impl ADDRIVar1 {
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
        self.oprx8_8_X.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 6i128 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 0u64 as u16;
        let oprx8_8_X = if let Some((len, table)) = Tableoprx8_8_X::parse(
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
        Some((pattern_len, Self { oprx8_8_X }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:159:1, end:159:6))"]
#[derive(Clone, Debug)]
struct ADDRIVar2 {
    comma_X: Tablecomma_X,
}
impl ADDRIVar2 {
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
        self.comma_X.display_extend(
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
        if token_parser.TokenFieldop4_6().disassembly() != 7i128 {
            return None;
        }
        let comma_X = if let Some((len, table)) = Tablecomma_X::parse(
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
        Some((pattern_len, Self { comma_X }))
    }
}
#[derive(Clone, Debug)]
enum TableADDRI {
    Var0(ADDRIVar0),
    Var1(ADDRIVar1),
    Var2(ADDRIVar2),
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
            ADDRIVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ADDRIVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            ADDRIVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:161:1, end:161:4))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/HCS08/data/languages/HCS_HC.sinc, start:163:1, end:163:7))"]
#[derive(Clone, Debug)]
struct NthBitVar0 {
    nIndex: TokenField_nIndex,
}
impl NthBitVar0 {
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
        let mut calc_nthbit: i128 = 0;
        calc_nthbit = u32::try_from(self.nIndex.disassembly())
            .ok()
            .map(|shl| 1i128.checked_shl(shl))
            .flatten()
            .unwrap_or(0);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_nthbit)];
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
        let mut calc_nthbit: i128 = 0;
        let mut block_0_len = 1u64 as u16;
        let token_parser = <TokenParser<1usize>>::new(tokens_current)?;
        calc_nthbit =
            u32::try_from(token_parser.TokenFieldnIndex().disassembly())
                .ok()
                .map(|shl| 1i128.checked_shl(shl))
                .flatten()
                .unwrap_or(0);
        let nIndex = token_parser.TokenFieldnIndex();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { nIndex }))
    }
}
#[derive(Clone, Debug)]
enum TableNthBit {
    Var0(NthBitVar0),
}
impl TableNthBit {
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
            NthBitVar0::parse(tokens_param, &mut context_current, inst_start)
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
