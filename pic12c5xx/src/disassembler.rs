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
    i64: TryFrom<T>,
    <i64 as TryFrom<T>>::Error: core::fmt::Debug,
{
    DisplayElement::Number(hex, i64::try_from(num).unwrap())
}
#[derive(Clone, Copy, Debug)]
struct TokenField_op12(u16);
impl TokenField_op12 {
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
struct TokenField_op6(u8);
impl TokenField_op6 {
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
struct TokenField_op4(u8);
impl TokenField_op4 {
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
struct TokenField_op3(u8);
impl TokenField_op3 {
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
struct TokenField_d(u8);
impl TokenField_d {
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
struct TokenField_b3(u8);
impl TokenField_b3 {
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
struct TokenField_f5(u8);
impl TokenField_f5 {
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
struct TokenField_f5h(u8);
impl TokenField_f5h {
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
struct TokenField_k8(u8);
impl TokenField_k8 {
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
struct TokenField_k9(u16);
impl TokenField_k9 {
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
    fn TokenFieldop12(&self) -> TokenField_op12 {
        let inner_value = self.read_u16::<false>(0, 0, 12).unwrap();
        TokenField_op12(u16::try_from(inner_value).unwrap())
    }
    fn TokenFieldop6(&self) -> TokenField_op6 {
        let inner_value = self.read_u16::<false>(0, 6, 6).unwrap();
        TokenField_op6(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop4(&self) -> TokenField_op4 {
        let inner_value = self.read_u8::<false>(1, 0, 4).unwrap();
        TokenField_op4(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldop3(&self) -> TokenField_op3 {
        let inner_value = self.read_u8::<false>(1, 1, 3).unwrap();
        TokenField_op3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldd(&self) -> TokenField_d {
        let inner_value = self.read_u8::<false>(0, 5, 1).unwrap();
        TokenField_d(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldb3(&self) -> TokenField_b3 {
        let inner_value = self.read_u8::<false>(0, 5, 3).unwrap();
        TokenField_b3(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldf5(&self) -> TokenField_f5 {
        let inner_value = self.read_u8::<false>(0, 0, 5).unwrap();
        TokenField_f5(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldf5h(&self) -> TokenField_f5h {
        let inner_value = self.read_u8::<false>(0, 4, 1).unwrap();
        TokenField_f5h(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldk8(&self) -> TokenField_k8 {
        let inner_value = self.read_u8::<false>(0, 0, 8).unwrap();
        TokenField_k8(u8::try_from(inner_value).unwrap())
    }
    fn TokenFieldk9(&self) -> TokenField_k9 {
        let inner_value = self.read_u16::<false>(0, 0, 9).unwrap();
        TokenField_k9(u16::try_from(inner_value).unwrap())
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    PC,
    STKPTR,
    W,
    PCL,
    FSR,
    STATUS,
    PA,
    Z,
    DC,
    C,
    OPTION,
    INDF,
    TMR0,
    PCL_0,
    STATUS_0,
    FSR_0,
    OSCCAL,
    GPIO,
    TRIS,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PC => write!(f, "PC"),
            Self::STKPTR => write!(f, "STKPTR"),
            Self::W => write!(f, "W"),
            Self::PCL => write!(f, "PCL"),
            Self::FSR => write!(f, "FSR"),
            Self::STATUS => write!(f, "STATUS"),
            Self::PA => write!(f, "PA"),
            Self::Z => write!(f, "Z"),
            Self::DC => write!(f, "DC"),
            Self::C => write!(f, "C"),
            Self::OPTION => write!(f, "OPTION"),
            Self::INDF => write!(f, "INDF"),
            Self::TMR0 => write!(f, "TMR0"),
            Self::PCL_0 => write!(f, "PCL.0"),
            Self::STATUS_0 => write!(f, "STATUS.0"),
            Self::FSR_0 => write!(f, "FSR.0"),
            Self::OSCCAL => write!(f, "OSCCAL"),
            Self::GPIO => write!(f, "GPIO"),
            Self::TRIS => write!(f, "TRIS"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:407:1"]
#[derive(Clone, Debug)]
struct instructionVar0 {}
impl instructionVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CLRW")];
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
        if token_parser.TokenFieldop6().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldf5().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:414:1"]
#[derive(Clone, Debug)]
struct instructionVar1 {}
impl instructionVar1 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("CLRWDT")];
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
        if token_parser.TokenFieldop12().disassembly() != 4i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:532:1"]
#[derive(Clone, Debug)]
struct instructionVar2 {}
impl instructionVar2 {
    fn display_extend<T>(
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
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        if token_parser.TokenFieldop12().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:536:1"]
#[derive(Clone, Debug)]
struct instructionVar3 {}
impl instructionVar3 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("OPTION")];
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
        if token_parser.TokenFieldop12().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:578:1"]
#[derive(Clone, Debug)]
struct instructionVar4 {}
impl instructionVar4 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("SLEEP")];
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
        if token_parser.TokenFieldop12().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:249:1"]
#[derive(Clone, Debug)]
struct instructionVar5 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar5 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:257:1"]
#[derive(Clone, Debug)]
struct instructionVar6 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar6 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:265:1"]
#[derive(Clone, Debug)]
struct instructionVar7 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar7 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:273:1"]
#[derive(Clone, Debug)]
struct instructionVar8 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar8 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 5i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:281:1"]
#[derive(Clone, Debug)]
struct instructionVar9 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar9 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 4i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 6i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:297:1"]
#[derive(Clone, Debug)]
struct instructionVar10 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar10 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:305:1"]
#[derive(Clone, Debug)]
struct instructionVar11 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar11 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:313:1"]
#[derive(Clone, Debug)]
struct instructionVar12 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar12 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:321:1"]
#[derive(Clone, Debug)]
struct instructionVar13 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar13 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 5i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:329:1"]
#[derive(Clone, Debug)]
struct instructionVar14 {
    bit: Tablebit,
    status: Tablestatus,
}
impl instructionVar14 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 5i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 6i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
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
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:346:1"]
#[derive(Clone, Debug)]
struct instructionVar15 {
    bit: Tablebit,
    status: Tablestatus,
    skipInst: TableskipInst,
}
impl instructionVar15 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                status,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:352:1"]
#[derive(Clone, Debug)]
struct instructionVar16 {
    bit: Tablebit,
    status: Tablestatus,
    skipInst: TableskipInst,
}
impl instructionVar16 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                status,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:358:1"]
#[derive(Clone, Debug)]
struct instructionVar17 {
    bit: Tablebit,
    status: Tablestatus,
    skipInst: TableskipInst,
}
impl instructionVar17 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 6i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                status,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:373:1"]
#[derive(Clone, Debug)]
struct instructionVar18 {
    bit: Tablebit,
    status: Tablestatus,
    skipInst: TableskipInst,
}
impl instructionVar18 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 0i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                status,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:379:1"]
#[derive(Clone, Debug)]
struct instructionVar19 {
    bit: Tablebit,
    status: Tablestatus,
    skipInst: TableskipInst,
}
impl instructionVar19 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 1i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                status,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:385:1"]
#[derive(Clone, Debug)]
struct instructionVar20 {
    bit: Tablebit,
    status: Tablestatus,
    skipInst: TableskipInst,
}
impl instructionVar20 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.status.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 7i64 {
            return None;
        }
        if token_parser.TokenFieldb3().disassembly() != 2i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) = Tablestatus::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                status,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:399:1"]
#[derive(Clone, Debug)]
struct instructionVar21 {
    srcREG: TablesrcREG,
}
impl instructionVar21 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CLRF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 1i64 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:517:1"]
#[derive(Clone, Debug)]
struct instructionVar22 {
    srcREG: TablesrcREG,
}
impl instructionVar22 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 1i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12c5xx.slaspec:24:1"]
#[derive(Clone, Debug)]
struct instructionVar23 {
    trisREG: TabletrisREG,
}
impl instructionVar23 {
    fn display_extend<T>(
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
            DisplayElement::Literal("TRIS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.trisREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 0i64 {
            return None;
        }
        if token_parser.TokenFieldd().disassembly() != 0i64 {
            return None;
        }
        let trisREG = if let Some((len, table)) = TabletrisREG::parse(
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
        Some((pattern_len, Self { trisREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:196:1"]
#[derive(Clone, Debug)]
struct instructionVar24 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar24 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 7i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:209:1"]
#[derive(Clone, Debug)]
struct instructionVar25 {
    D: TableD,
    pcl: Tablepcl,
}
impl instructionVar25 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ADDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.pcl.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 7i64 {
            return None;
        }
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let pcl = if let Some((len, table)) =
            Tablepcl::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { D, pcl }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:230:1"]
#[derive(Clone, Debug)]
struct instructionVar26 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar26 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ANDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 5i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:420:1"]
#[derive(Clone, Debug)]
struct instructionVar27 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar27 {
    fn display_extend<T>(
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
            DisplayElement::Literal("COMF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 9i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:431:1"]
#[derive(Clone, Debug)]
struct instructionVar28 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar28 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DECF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 3i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:442:1"]
#[derive(Clone, Debug)]
struct instructionVar29 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
    skipInst: TableskipInst,
}
impl instructionVar29 {
    fn display_extend<T>(
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
            DisplayElement::Literal("DECFSZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 11i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                srcREG,
                D,
                destREG,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:460:1"]
#[derive(Clone, Debug)]
struct instructionVar30 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar30 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INCF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 10i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:471:1"]
#[derive(Clone, Debug)]
struct instructionVar31 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
    skipInst: TableskipInst,
}
impl instructionVar31 {
    fn display_extend<T>(
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
            DisplayElement::Literal("INCFSZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 15i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                srcREG,
                D,
                destREG,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:489:1"]
#[derive(Clone, Debug)]
struct instructionVar32 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar32 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IORWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 4i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:506:1"]
#[derive(Clone, Debug)]
struct instructionVar33 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar33 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 8i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:524:1"]
#[derive(Clone, Debug)]
struct instructionVar34 {
    pcl: Tablepcl,
}
impl instructionVar34 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.pcl.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 0i64 {
            return None;
        }
        let pcl = if let Some((len, table)) =
            Tablepcl::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { pcl }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:550:1"]
#[derive(Clone, Debug)]
struct instructionVar35 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar35 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RLF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 13i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:564:1"]
#[derive(Clone, Debug)]
struct instructionVar36 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar36 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("RRF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 12i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:584:1"]
#[derive(Clone, Debug)]
struct instructionVar37 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar37 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SUBWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 2i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:597:1"]
#[derive(Clone, Debug)]
struct instructionVar38 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar38 {
    fn display_extend<T>(
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
            DisplayElement::Literal("SWAPF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 14i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:614:1"]
#[derive(Clone, Debug)]
struct instructionVar39 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl instructionVar39 {
    fn display_extend<T>(
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
            DisplayElement::Literal("XORWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.D.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop6().disassembly() != 6i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) = TabledestREG::parse(
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
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:223:1"]
#[derive(Clone, Debug)]
struct instructionVar40 {
    imm8: Tableimm8,
}
impl instructionVar40 {
    fn display_extend<T>(
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
            DisplayElement::Literal("ANDLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 14i64 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:241:1"]
#[derive(Clone, Debug)]
struct instructionVar41 {
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl instructionVar41 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 4i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:289:1"]
#[derive(Clone, Debug)]
struct instructionVar42 {
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl instructionVar42 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 5i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:337:1"]
#[derive(Clone, Debug)]
struct instructionVar43 {
    bit: Tablebit,
    srcREG: TablesrcREG,
    skipInst: TableskipInst,
}
impl instructionVar43 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 6i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                srcREG,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:364:1"]
#[derive(Clone, Debug)]
struct instructionVar44 {
    bit: Tablebit,
    srcREG: TablesrcREG,
    skipInst: TableskipInst,
}
impl instructionVar44 {
    fn display_extend<T>(
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
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.srcREG.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        self.bit.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 7i64 {
            return None;
        }
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
            tokens_current,
            &mut context_instance,
            inst_start,
        ) {
            block_0_len = block_0_len.max(len as u16);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) = TableskipInst::parse(
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
        Some((
            pattern_len,
            Self {
                bit,
                srcREG,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:391:1"]
#[derive(Clone, Debug)]
struct instructionVar45 {
    absAddr8: TableabsAddr8,
}
impl instructionVar45 {
    fn display_extend<T>(
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
            DisplayElement::Literal("CALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.absAddr8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 9i64 {
            return None;
        }
        let absAddr8 = if let Some((len, table)) = TableabsAddr8::parse(
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
        Some((pattern_len, Self { absAddr8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:482:1"]
#[derive(Clone, Debug)]
struct instructionVar46 {
    imm8: Tableimm8,
}
impl instructionVar46 {
    fn display_extend<T>(
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
            DisplayElement::Literal("IORLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 13i64 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:500:1"]
#[derive(Clone, Debug)]
struct instructionVar47 {
    imm8: Tableimm8,
}
impl instructionVar47 {
    fn display_extend<T>(
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
            DisplayElement::Literal("MOVLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 12i64 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:541:1"]
#[derive(Clone, Debug)]
struct instructionVar48 {
    imm8: Tableimm8,
}
impl instructionVar48 {
    fn display_extend<T>(
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
            DisplayElement::Literal("RETLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 8i64 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:607:1"]
#[derive(Clone, Debug)]
struct instructionVar49 {
    imm8: Tableimm8,
}
impl instructionVar49 {
    fn display_extend<T>(
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
            DisplayElement::Literal("XORLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.imm8.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop4().disassembly() != 15i64 {
            return None;
        }
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
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
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:453:1"]
#[derive(Clone, Debug)]
struct instructionVar50 {
    absAddr9: TableabsAddr9,
}
impl instructionVar50 {
    fn display_extend<T>(
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
            DisplayElement::Literal("GOTO"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.absAddr9.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        if token_parser.TokenFieldop3().disassembly() != 5i64 {
            return None;
        }
        let absAddr9 = if let Some((len, table)) = TableabsAddr9::parse(
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
        Some((pattern_len, Self { absAddr9 }))
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
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:120:1"]
#[derive(Clone, Debug)]
struct fREGLocVar0 {}
impl fREGLocVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("INDF")];
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
        if token_parser.TokenFieldf5().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:130:1"]
#[derive(Clone, Debug)]
struct fREGLocVar1 {}
impl fREGLocVar1 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("STATUS")];
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
        if token_parser.TokenFieldf5().disassembly() != 3i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:131:1"]
#[derive(Clone, Debug)]
struct fREGLocVar2 {}
impl fREGLocVar2 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("FSR")];
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
        if token_parser.TokenFieldf5().disassembly() != 4i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:132:1"]
#[derive(Clone, Debug)]
struct fREGLocVar3 {}
impl fREGLocVar3 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PCL")];
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
        if token_parser.TokenFieldf5().disassembly() != 2i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:126:1"]
#[derive(Clone, Debug)]
struct fREGLocVar4 {
    f5: TokenField_f5,
}
impl fREGLocVar4 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.f5.display()];
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
        if token_parser.TokenFieldf5h().disassembly() != 0i64 {
            return None;
        }
        let f5 = token_parser.TokenFieldf5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { f5 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:114:1"]
#[derive(Clone, Debug)]
struct fREGLocVar5 {
    f5: TokenField_f5,
}
impl fREGLocVar5 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.f5.display()];
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
        let f5 = token_parser.TokenFieldf5();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { f5 }))
    }
}
#[derive(Clone, Debug)]
enum TablefREGLoc {
    Var0(fREGLocVar0),
    Var1(fREGLocVar1),
    Var2(fREGLocVar2),
    Var3(fREGLocVar3),
    Var4(fREGLocVar4),
    Var5(fREGLocVar5),
}
impl TablefREGLoc {
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
            fREGLocVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            fREGLocVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        if let Some((inst_len, parsed)) =
            fREGLocVar2::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var2(parsed)));
        }
        if let Some((inst_len, parsed)) =
            fREGLocVar3::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var3(parsed)));
        }
        if let Some((inst_len, parsed)) =
            fREGLocVar4::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var4(parsed)));
        }
        if let Some((inst_len, parsed)) =
            fREGLocVar5::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var5(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:135:1"]
#[derive(Clone, Debug)]
struct srcREGVar0 {
    fREGLoc: TablefREGLoc,
}
impl srcREGVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        self.fREGLoc.display_extend(
            display, context, inst_start, inst_next, global_set,
        );
    }
    fn parse<T>(
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
        let fREGLoc = if let Some((len, table)) = TablefREGLoc::parse(
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
        Some((pattern_len, Self { fREGLoc }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:138:1"]
#[derive(Clone, Debug)]
struct srcREGVar1 {}
impl srcREGVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PCL")];
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
        if token_parser.TokenFieldf5().disassembly() != 2i64 {
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
enum TablesrcREG {
    Var0(srcREGVar0),
    Var1(srcREGVar1),
}
impl TablesrcREG {
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
            srcREGVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            srcREGVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:148:1"]
#[derive(Clone, Debug)]
struct destREGVar0 {}
impl destREGVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("0")];
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
        if token_parser.TokenFieldd().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:149:1"]
#[derive(Clone, Debug)]
struct destREGVar1 {
    srcREG: TablesrcREG,
}
impl destREGVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("1")];
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
        if token_parser.TokenFieldd().disassembly() != 1i64 {
            return None;
        }
        let srcREG = if let Some((len, table)) = TablesrcREG::parse(
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
        Some((pattern_len, Self { srcREG }))
    }
}
#[derive(Clone, Debug)]
enum TabledestREG {
    Var0(destREGVar0),
    Var1(destREGVar1),
}
impl TabledestREG {
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
            destREGVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            destREGVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:164:1"]
#[derive(Clone, Debug)]
struct DVar0 {}
impl DVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("w")];
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
        if token_parser.TokenFieldd().disassembly() != 0i64 {
            return None;
        }
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:165:1"]
#[derive(Clone, Debug)]
struct DVar1 {}
impl DVar1 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("f")];
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
        if token_parser.TokenFieldd().disassembly() != 1i64 {
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
enum TableD {
    Var0(DVar0),
    Var1(DVar1),
}
impl TableD {
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
            DVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        if let Some((inst_len, parsed)) =
            DVar1::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:168:1"]
#[derive(Clone, Debug)]
struct absAddr8Var0 {
    k8: TokenField_k8,
}
impl absAddr8Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.k8.display()];
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
        let k8 = token_parser.TokenFieldk8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k8 }))
    }
}
#[derive(Clone, Debug)]
enum TableabsAddr8 {
    Var0(absAddr8Var0),
}
impl TableabsAddr8 {
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
            absAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:172:1"]
#[derive(Clone, Debug)]
struct absAddr9Var0 {
    k9: TokenField_k9,
}
impl absAddr9Var0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [self.k9.display()];
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
        let k9 = token_parser.TokenFieldk9();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k9 }))
    }
}
#[derive(Clone, Debug)]
enum TableabsAddr9 {
    Var0(absAddr9Var0),
}
impl TableabsAddr9 {
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
            absAddr9Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:178:1"]
#[derive(Clone, Debug)]
struct skipInstVar0 {}
impl skipInstVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let mut calc_inst_skip: i64 = 0;
        calc_inst_skip = i64::try_from(inst_next).unwrap().wrapping_add(1i64);
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, calc_inst_skip)];
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
        let mut calc_inst_skip: i64 = 0;
        let mut block_0_len = 2u64 as u16;
        let token_parser = <TokenParser<2usize>>::new(tokens_current)?;
        let op12 = token_parser.TokenFieldop12();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableskipInst {
    Var0(skipInstVar0),
}
impl TableskipInst {
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
            skipInstVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:181:1"]
#[derive(Clone, Debug)]
struct imm8Var0 {
    k8: TokenField_k8,
}
impl imm8Var0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.k8.display()];
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
        let k8 = token_parser.TokenFieldk8();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k8 }))
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
            imm8Var0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:184:1"]
#[derive(Clone, Debug)]
struct bitVar0 {
    b3: TokenField_b3,
}
impl bitVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("#"), self.b3.display()];
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
        let b3 = token_parser.TokenFieldb3();
        pattern_len += block_0_len;
        tokens_current =
            &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { b3 }))
    }
}
#[derive(Clone, Debug)]
enum Tablebit {
    Var0(bitVar0),
}
impl Tablebit {
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
            bitVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:187:1"]
#[derive(Clone, Debug)]
struct pclVar0 {}
impl pclVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PC")];
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
        if token_parser.TokenFieldf5().disassembly() != 2i64 {
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
enum Tablepcl {
    Var0(pclVar0),
}
impl Tablepcl {
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
            pclVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:190:1"]
#[derive(Clone, Debug)]
struct statusVar0 {}
impl statusVar0 {
    fn display_extend<T>(
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
            [DisplayElement::Literal("STATUS")];
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
        if token_parser.TokenFieldf5().disassembly() != 3i64 {
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
enum Tablestatus {
    Var0(statusVar0),
}
impl Tablestatus {
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
            statusVar0::parse(tokens_param, &mut context_current, inst_start)
        {
            *context_param = context_current;
            return Some((inst_len, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12c5xx.slaspec:22:1"]
#[derive(Clone, Debug)]
struct trisREGVar0 {}
impl trisREGVar0 {
    fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("6")];
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
        if token_parser.TokenFieldf5().disassembly() != 6i64 {
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
enum TabletrisREG {
    Var0(trisREGVar0),
}
impl TabletrisREG {
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
            trisREGVar0::parse(tokens_param, &mut context_current, inst_start)
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
