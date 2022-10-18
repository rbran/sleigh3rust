pub struct TokenParser16([u8; 2usize]);
impl TokenParser16 {
    pub fn new(data: &[u8]) -> Option<Self> {
        <[u8; 2usize]>::try_from(data).ok().map(Self)
    }
    pub fn d(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 5u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn op12(&self) -> u16 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 4095u128 as u16;
        raw_value as u16
    }
    pub fn k9(&self) -> u16 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 511u128 as u16;
        raw_value as u16
    }
    pub fn op4(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn op3(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 9u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn f5(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 31u128 as u16;
        raw_value as u8
    }
    pub fn op6(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 6u64 as u16;
        raw_value &= 63u128 as u16;
        raw_value as u8
    }
    pub fn k8(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 255u128 as u16;
        raw_value as u8
    }
    pub fn f5h(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn b3(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 5u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
}
pub trait GlobalSetTrait {}
pub trait ContextTrait {}
#[derive(Debug, Clone, Copy)]
pub struct Context {}
impl ContextTrait for Context {}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    Z,
    STATUS,
    W,
    OSCCAL,
    PCL_0,
    STATUS_0,
    PCL,
    STKPTR,
    C,
    TMR0,
    INDF,
    FSR_0,
    OPTION,
    DC,
    TRIS,
    PA,
    FSR,
    PC,
    GPIO,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Z => write!(f, "Z"),
            Self::STATUS => write!(f, "STATUS"),
            Self::W => write!(f, "W"),
            Self::OSCCAL => write!(f, "OSCCAL"),
            Self::PCL_0 => write!(f, "PCL.0"),
            Self::STATUS_0 => write!(f, "STATUS.0"),
            Self::PCL => write!(f, "PCL"),
            Self::STKPTR => write!(f, "STKPTR"),
            Self::C => write!(f, "C"),
            Self::TMR0 => write!(f, "TMR0"),
            Self::INDF => write!(f, "INDF"),
            Self::FSR_0 => write!(f, "FSR.0"),
            Self::OPTION => write!(f, "OPTION"),
            Self::DC => write!(f, "DC"),
            Self::TRIS => write!(f, "TRIS"),
            Self::PA => write!(f, "PA"),
            Self::FSR => write!(f, "FSR"),
            Self::PC => write!(f, "PC"),
            Self::GPIO => write!(f, "GPIO"),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum DisplayElement {
    Literal(&'static str),
    Register(Register),
    Signed(bool, i64),
    Unsigned(bool, u64),
}
impl core::fmt::Display for DisplayElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(lit) => lit.fmt(f),
            Self::Register(reg) => reg.fmt(f),
            Self::Signed(hex, value) => {
                if *hex {
                    write!(f, "0x{:x}", value)
                } else {
                    value.fmt(f)
                }
            }
            Self::Unsigned(hex, value) => {
                if *hex {
                    write!(f, "0x{:x}", value)
                } else {
                    value.fmt(f)
                }
            }
        }
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:184:1"]
#[derive(Clone, Debug)]
pub struct bitVar0 {
    b3: u8,
}
impl bitVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { b3 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Unsigned(true, u64::try_from(*b3).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            let b3 = token_parser.b3();
            *context = context_current;
            Some(((), (b3), block_len))
        };
        let ((), (b3), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { b3 }))
    }
}
#[doc = "Table bit"]
#[derive(Clone, Debug)]
pub enum bit {
    Var0(bitVar0),
}
impl bit {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = bitVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:578:1"]
#[derive(Clone, Debug)]
pub struct instructionVar0 {}
impl instructionVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("SLEEP")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op12() != (3u64 as i64) as u16 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:536:1"]
#[derive(Clone, Debug)]
pub struct instructionVar1 {}
impl instructionVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("OPTION")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op12() != (2u64 as i64) as u16 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:532:1"]
#[derive(Clone, Debug)]
pub struct instructionVar2 {}
impl instructionVar2 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("NOP")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op12() != (0u64 as i64) as u16 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:414:1"]
#[derive(Clone, Debug)]
pub struct instructionVar3 {}
impl instructionVar3 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("CLRWDT")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op12() != (4u64 as i64) as u16 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:614:1"]
#[derive(Clone, Debug)]
pub struct instructionVar4 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar4 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XORWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (6u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((D, destREG, srcREG), (), block_len))
        };
        let ((mut D, mut destREG, mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:597:1"]
#[derive(Clone, Debug)]
pub struct instructionVar5 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar5 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SWAPF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (14u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((D, destREG, srcREG), (), block_len))
        };
        let ((mut D, mut destREG, mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:584:1"]
#[derive(Clone, Debug)]
pub struct instructionVar6 {
    D: D,
    srcREG: srcREG,
}
impl instructionVar6 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { D, srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("SUBWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (2u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, D, destREG), (), block_len))
        };
        let ((mut srcREG, mut D, mut destREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { D, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:564:1"]
#[derive(Clone, Debug)]
pub struct instructionVar7 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar7 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("RRF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (12u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, destREG, D), (), block_len))
        };
        let ((mut srcREG, mut destREG, mut D), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:550:1"]
#[derive(Clone, Debug)]
pub struct instructionVar8 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar8 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("RLF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (13u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((destREG, srcREG, D), (), block_len))
        };
        let ((mut destREG, mut srcREG, mut D), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:524:1"]
#[derive(Clone, Debug)]
pub struct instructionVar9 {
    pcl: pcl,
}
impl instructionVar9 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { pcl } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        pcl.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (0u64 as i64) as u8 {
                return None;
            }
            let pcl = if let Some((len, table)) =
                pcl::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((pcl), (), block_len))
        };
        let ((mut pcl), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        pcl.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { pcl }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:506:1"]
#[derive(Clone, Debug)]
pub struct instructionVar10 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar10 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (8u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, D, destREG), (), block_len))
        };
        let ((mut srcREG, mut D, mut destREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:489:1"]
#[derive(Clone, Debug)]
pub struct instructionVar11 {
    D: D,
    srcREG: srcREG,
}
impl instructionVar11 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { D, srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("IORWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (4u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((D, destREG, srcREG), (), block_len))
        };
        let ((mut D, mut destREG, mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { D, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:471:1"]
#[derive(Clone, Debug)]
pub struct instructionVar12 {
    D: D,
    srcREG: srcREG,
}
impl instructionVar12 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { D, srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("INCFSZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (15u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((D, destREG, srcREG, skipInst), (), block_len))
        };
        let ((mut D, mut destREG, mut srcREG, mut skipInst), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { D, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:460:1"]
#[derive(Clone, Debug)]
pub struct instructionVar13 {
    D: D,
    srcREG: srcREG,
}
impl instructionVar13 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { D, srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("INCF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (10u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, D, destREG), (), block_len))
        };
        let ((mut srcREG, mut D, mut destREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { D, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:442:1"]
#[derive(Clone, Debug)]
pub struct instructionVar14 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar14 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DECFSZ"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (11u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((D, destREG, skipInst, srcREG), (), block_len))
        };
        let ((mut D, mut destREG, mut skipInst, mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:431:1"]
#[derive(Clone, Debug)]
pub struct instructionVar15 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar15 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("DECF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (3u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, D, destREG), (), block_len))
        };
        let ((mut srcREG, mut D, mut destREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:420:1"]
#[derive(Clone, Debug)]
pub struct instructionVar16 {
    D: D,
    srcREG: srcREG,
}
impl instructionVar16 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { D, srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("COMF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (9u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, D, destREG), (), block_len))
        };
        let ((mut srcREG, mut D, mut destREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { D, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:230:1"]
#[derive(Clone, Debug)]
pub struct instructionVar17 {
    D: D,
    srcREG: srcREG,
}
impl instructionVar17 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { D, srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ANDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (5u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, D, destREG), (), block_len))
        };
        let ((mut srcREG, mut D, mut destREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { D, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:209:1"]
#[derive(Clone, Debug)]
pub struct instructionVar18 {
    D: D,
    pcl: pcl,
}
impl instructionVar18 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { D, pcl } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        pcl.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (7u64 as i64) as u8 {
                return None;
            }
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let pcl = if let Some((len, table)) =
                pcl::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((D, pcl), (), block_len))
        };
        let ((mut D, mut pcl), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        pcl.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { D, pcl }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:196:1"]
#[derive(Clone, Debug)]
pub struct instructionVar19 {
    srcREG: srcREG,
    D: D,
}
impl instructionVar19 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, D } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ADDWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        D.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (7u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let D = if let Some((len, table)) =
                D::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let destREG = if let Some((len, table)) = destREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, D, destREG), (), block_len))
        };
        let ((mut srcREG, mut D, mut destREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        D.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { srcREG, D }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:607:1"]
#[derive(Clone, Debug)]
pub struct instructionVar20 {
    imm8: imm8,
}
impl instructionVar20 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { imm8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("XORLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        imm8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (15u64 as i64) as u8 {
                return None;
            }
            let imm8 = if let Some((len, table)) = imm8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((imm8), (), block_len))
        };
        let ((mut imm8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        imm8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:541:1"]
#[derive(Clone, Debug)]
pub struct instructionVar21 {
    imm8: imm8,
}
impl instructionVar21 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { imm8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("RETLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        imm8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (8u64 as i64) as u8 {
                return None;
            }
            let imm8 = if let Some((len, table)) = imm8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((imm8), (), block_len))
        };
        let ((mut imm8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        imm8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:500:1"]
#[derive(Clone, Debug)]
pub struct instructionVar22 {
    imm8: imm8,
}
impl instructionVar22 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { imm8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        imm8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (12u64 as i64) as u8 {
                return None;
            }
            let imm8 = if let Some((len, table)) = imm8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((imm8), (), block_len))
        };
        let ((mut imm8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        imm8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:482:1"]
#[derive(Clone, Debug)]
pub struct instructionVar23 {
    imm8: imm8,
}
impl instructionVar23 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { imm8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("IORLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        imm8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (13u64 as i64) as u8 {
                return None;
            }
            let imm8 = if let Some((len, table)) = imm8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((imm8), (), block_len))
        };
        let ((mut imm8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        imm8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:391:1"]
#[derive(Clone, Debug)]
pub struct instructionVar24 {
    absAddr8: absAddr8,
}
impl instructionVar24 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { absAddr8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("CALL"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        absAddr8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (9u64 as i64) as u8 {
                return None;
            }
            let absAddr8 = if let Some((len, table)) = absAddr8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((absAddr8), (), block_len))
        };
        let ((mut absAddr8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        absAddr8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { absAddr8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:364:1"]
#[derive(Clone, Debug)]
pub struct instructionVar25 {
    srcREG: srcREG,
    bit: bit,
}
impl instructionVar25 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, bit } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (7u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, srcREG, skipInst), (), block_len))
        };
        let ((mut bit, mut srcREG, mut skipInst), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { srcREG, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:337:1"]
#[derive(Clone, Debug)]
pub struct instructionVar26 {
    bit: bit,
    srcREG: srcREG,
}
impl instructionVar26 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (6u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((skipInst, bit, srcREG), (), block_len))
        };
        let ((mut skipInst, mut bit, mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:289:1"]
#[derive(Clone, Debug)]
pub struct instructionVar27 {
    srcREG: srcREG,
    bit: bit,
}
impl instructionVar27 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, bit } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (5u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, srcREG), (), block_len))
        };
        let ((mut bit, mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { srcREG, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:241:1"]
#[derive(Clone, Debug)]
pub struct instructionVar28 {
    srcREG: srcREG,
    bit: bit,
}
impl instructionVar28 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG, bit } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (4u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG, bit), (), block_len))
        };
        let ((mut srcREG, mut bit), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { srcREG, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:223:1"]
#[derive(Clone, Debug)]
pub struct instructionVar29 {
    imm8: imm8,
}
impl instructionVar29 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { imm8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ANDLW"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        imm8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (14u64 as i64) as u8 {
                return None;
            }
            let imm8 = if let Some((len, table)) = imm8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((imm8), (), block_len))
        };
        let ((mut imm8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        imm8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { imm8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12c5xx.slaspec:24:1"]
#[derive(Clone, Debug)]
pub struct instructionVar30 {
    trisREG: trisREG,
}
impl instructionVar30 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { trisREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("TRIS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        trisREG.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (0u64 as i64) as u8 {
                return None;
            }
            if token_parser.d() != (0u64 as i64) as u8 {
                return None;
            }
            let trisREG = if let Some((len, table)) = trisREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((trisREG), (), block_len))
        };
        let ((mut trisREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        trisREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { trisREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:517:1"]
#[derive(Clone, Debug)]
pub struct instructionVar31 {
    srcREG: srcREG,
}
impl instructionVar31 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("MOVWF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (0u64 as i64) as u8 {
                return None;
            }
            if token_parser.d() != (1u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG), (), block_len))
        };
        let ((mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:399:1"]
#[derive(Clone, Debug)]
pub struct instructionVar32 {
    srcREG: srcREG,
}
impl instructionVar32 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { srcREG } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("CLRF"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        srcREG.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (1u64 as i64) as u8 {
                return None;
            }
            if token_parser.d() != (1u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG), (), block_len))
        };
        let ((mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        srcREG.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { srcREG }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:385:1"]
#[derive(Clone, Debug)]
pub struct instructionVar33 {
    status: status,
    bit: bit,
}
impl instructionVar33 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (7u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (2u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status, skipInst), (), block_len))
        };
        let ((mut bit, mut status, mut skipInst), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:379:1"]
#[derive(Clone, Debug)]
pub struct instructionVar34 {
    status: status,
    bit: bit,
}
impl instructionVar34 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (7u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (1u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((status, skipInst, bit), (), block_len))
        };
        let ((mut status, mut skipInst, mut bit), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:373:1"]
#[derive(Clone, Debug)]
pub struct instructionVar35 {
    status: status,
    bit: bit,
}
impl instructionVar35 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSS"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (7u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (0u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, skipInst, status), (), block_len))
        };
        let ((mut bit, mut skipInst, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:358:1"]
#[derive(Clone, Debug)]
pub struct instructionVar36 {
    status: status,
    bit: bit,
}
impl instructionVar36 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (6u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (2u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, skipInst, status), (), block_len))
        };
        let ((mut bit, mut skipInst, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:352:1"]
#[derive(Clone, Debug)]
pub struct instructionVar37 {
    status: status,
    bit: bit,
}
impl instructionVar37 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (6u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (1u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((status, skipInst, bit), (), block_len))
        };
        let ((mut status, mut skipInst, mut bit), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:346:1"]
#[derive(Clone, Debug)]
pub struct instructionVar38 {
    bit: bit,
    status: status,
}
impl instructionVar38 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, status } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("BTFSC"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (6u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (0u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let skipInst = if let Some((len, table)) = skipInst::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((status, skipInst, bit), (), block_len))
        };
        let ((mut status, mut skipInst, mut bit), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:329:1"]
#[derive(Clone, Debug)]
pub struct instructionVar39 {
    status: status,
    bit: bit,
}
impl instructionVar39 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (5u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (6u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:321:1"]
#[derive(Clone, Debug)]
pub struct instructionVar40 {
    bit: bit,
    status: status,
}
impl instructionVar40 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, status } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (5u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (5u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:313:1"]
#[derive(Clone, Debug)]
pub struct instructionVar41 {
    bit: bit,
    status: status,
}
impl instructionVar41 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, status } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (5u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (2u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:305:1"]
#[derive(Clone, Debug)]
pub struct instructionVar42 {
    bit: bit,
    status: status,
}
impl instructionVar42 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, status } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (5u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (1u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:297:1"]
#[derive(Clone, Debug)]
pub struct instructionVar43 {
    bit: bit,
    status: status,
}
impl instructionVar43 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, status } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BSF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (5u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (0u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((status, bit), (), block_len))
        };
        let ((mut status, mut bit), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:281:1"]
#[derive(Clone, Debug)]
pub struct instructionVar44 {
    status: status,
    bit: bit,
}
impl instructionVar44 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (4u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (6u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:273:1"]
#[derive(Clone, Debug)]
pub struct instructionVar45 {
    bit: bit,
    status: status,
}
impl instructionVar45 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, status } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (4u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (5u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((status, bit), (), block_len))
        };
        let ((mut status, mut bit), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:265:1"]
#[derive(Clone, Debug)]
pub struct instructionVar46 {
    bit: bit,
    status: status,
}
impl instructionVar46 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { bit, status } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (4u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (2u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { bit, status }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:257:1"]
#[derive(Clone, Debug)]
pub struct instructionVar47 {
    status: status,
    bit: bit,
}
impl instructionVar47 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (4u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (1u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:249:1"]
#[derive(Clone, Debug)]
pub struct instructionVar48 {
    status: status,
    bit: bit,
}
impl instructionVar48 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { status, bit } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("BCF"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        status.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(", ")];
        display.extend_from_slice(&extend);
        bit.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op4() != (4u64 as i64) as u8 {
                return None;
            }
            if token_parser.b3() != (0u64 as i64) as u8 {
                return None;
            }
            let bit = if let Some((len, table)) =
                bit::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            let status = if let Some((len, table)) = status::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((bit, status), (), block_len))
        };
        let ((mut bit, mut status), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        status.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        bit.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { status, bit }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:407:1"]
#[derive(Clone, Debug)]
pub struct instructionVar49 {}
impl instructionVar49 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("CLRW")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op6() != (1u64 as i64) as u8 {
                return None;
            }
            if token_parser.d() != (0u64 as i64) as u8 {
                return None;
            }
            if token_parser.f5() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:453:1"]
#[derive(Clone, Debug)]
pub struct instructionVar50 {
    absAddr9: absAddr9,
}
impl instructionVar50 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { absAddr9 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("GOTO"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        absAddr9.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.op3() != (5u64 as i64) as u8 {
                return None;
            }
            let absAddr9 = if let Some((len, table)) = absAddr9::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((absAddr9), (), block_len))
        };
        let ((mut absAddr9), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        absAddr9.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { absAddr9 }))
    }
}
#[doc = "Table instruction"]
#[derive(Clone, Debug)]
pub enum instruction {
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
impl instruction {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
            Self::Var1(x) => x.display_extend(display, context),
            Self::Var2(x) => x.display_extend(display, context),
            Self::Var3(x) => x.display_extend(display, context),
            Self::Var4(x) => x.display_extend(display, context),
            Self::Var5(x) => x.display_extend(display, context),
            Self::Var6(x) => x.display_extend(display, context),
            Self::Var7(x) => x.display_extend(display, context),
            Self::Var8(x) => x.display_extend(display, context),
            Self::Var9(x) => x.display_extend(display, context),
            Self::Var10(x) => x.display_extend(display, context),
            Self::Var11(x) => x.display_extend(display, context),
            Self::Var12(x) => x.display_extend(display, context),
            Self::Var13(x) => x.display_extend(display, context),
            Self::Var14(x) => x.display_extend(display, context),
            Self::Var15(x) => x.display_extend(display, context),
            Self::Var16(x) => x.display_extend(display, context),
            Self::Var17(x) => x.display_extend(display, context),
            Self::Var18(x) => x.display_extend(display, context),
            Self::Var19(x) => x.display_extend(display, context),
            Self::Var20(x) => x.display_extend(display, context),
            Self::Var21(x) => x.display_extend(display, context),
            Self::Var22(x) => x.display_extend(display, context),
            Self::Var23(x) => x.display_extend(display, context),
            Self::Var24(x) => x.display_extend(display, context),
            Self::Var25(x) => x.display_extend(display, context),
            Self::Var26(x) => x.display_extend(display, context),
            Self::Var27(x) => x.display_extend(display, context),
            Self::Var28(x) => x.display_extend(display, context),
            Self::Var29(x) => x.display_extend(display, context),
            Self::Var30(x) => x.display_extend(display, context),
            Self::Var31(x) => x.display_extend(display, context),
            Self::Var32(x) => x.display_extend(display, context),
            Self::Var33(x) => x.display_extend(display, context),
            Self::Var34(x) => x.display_extend(display, context),
            Self::Var35(x) => x.display_extend(display, context),
            Self::Var36(x) => x.display_extend(display, context),
            Self::Var37(x) => x.display_extend(display, context),
            Self::Var38(x) => x.display_extend(display, context),
            Self::Var39(x) => x.display_extend(display, context),
            Self::Var40(x) => x.display_extend(display, context),
            Self::Var41(x) => x.display_extend(display, context),
            Self::Var42(x) => x.display_extend(display, context),
            Self::Var43(x) => x.display_extend(display, context),
            Self::Var44(x) => x.display_extend(display, context),
            Self::Var45(x) => x.display_extend(display, context),
            Self::Var46(x) => x.display_extend(display, context),
            Self::Var47(x) => x.display_extend(display, context),
            Self::Var48(x) => x.display_extend(display, context),
            Self::Var49(x) => x.display_extend(display, context),
            Self::Var50(x) => x.display_extend(display, context),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = instructionVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var1(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var2(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var3(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var4(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var5(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var6(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var7(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar8::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var8(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar9::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var9(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar10::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var10(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar11::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var11(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar12::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var12(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar13::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var13(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar14::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var14(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar15::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var15(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar16::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var16(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar17::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var17(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar18::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var18(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar19::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var19(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar20::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var20(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar21::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var21(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar22::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var22(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar23::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var23(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar24::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var24(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar25::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var25(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar26::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var26(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar27::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var27(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar28::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var28(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar29::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var29(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar30::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var30(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar31::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var31(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar32::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var32(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar33::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var33(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar34::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var34(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar35::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var35(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar36::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var36(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar37::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var37(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar38::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var38(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar39::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var39(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar40::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var40(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar41::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var41(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar42::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var42(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar43::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var43(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar44::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var44(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar45::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var45(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar46::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var46(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar47::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var47(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar48::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var48(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar49::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var49(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar50::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var50(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:132:1"]
#[derive(Clone, Debug)]
pub struct fREGLocVar0 {}
impl fREGLocVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PCL")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:131:1"]
#[derive(Clone, Debug)]
pub struct fREGLocVar1 {}
impl fREGLocVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("FSR")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:130:1"]
#[derive(Clone, Debug)]
pub struct fREGLocVar2 {}
impl fREGLocVar2 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STATUS")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:120:1"]
#[derive(Clone, Debug)]
pub struct fREGLocVar3 {}
impl fREGLocVar3 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("INDF")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:114:1"]
#[derive(Clone, Debug)]
pub struct fREGLocVar4 {
    f5: u8,
}
impl fREGLocVar4 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { f5 } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Unsigned(true, u64::try_from(*f5).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            let f5 = token_parser.f5();
            *context = context_current;
            Some(((), (f5), block_len))
        };
        let ((), (f5), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { f5 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:126:1"]
#[derive(Clone, Debug)]
pub struct fREGLocVar5 {
    f5: u8,
}
impl fREGLocVar5 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { f5 } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Unsigned(true, u64::try_from(*f5).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            let f5 = token_parser.f5();
            if token_parser.f5h() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (f5), block_len))
        };
        let ((), (f5), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { f5 }))
    }
}
#[doc = "Table fREGLoc"]
#[derive(Clone, Debug)]
pub enum fREGLoc {
    Var0(fREGLocVar0),
    Var1(fREGLocVar1),
    Var2(fREGLocVar2),
    Var3(fREGLocVar3),
    Var4(fREGLocVar4),
    Var5(fREGLocVar5),
}
impl fREGLoc {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
            Self::Var1(x) => x.display_extend(display, context),
            Self::Var2(x) => x.display_extend(display, context),
            Self::Var3(x) => x.display_extend(display, context),
            Self::Var4(x) => x.display_extend(display, context),
            Self::Var5(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var2(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var3(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var4(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var5(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = fREGLocVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        if let Some((inst_next, parsed)) = fREGLocVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var1(parsed)));
        }
        if let Some((inst_next, parsed)) = fREGLocVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var2(parsed)));
        }
        if let Some((inst_next, parsed)) = fREGLocVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var3(parsed)));
        }
        if let Some((inst_next, parsed)) = fREGLocVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var4(parsed)));
        }
        if let Some((inst_next, parsed)) = fREGLocVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var5(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:190:1"]
#[derive(Clone, Debug)]
pub struct statusVar0 {}
impl statusVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("STATUS")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table status"]
#[derive(Clone, Debug)]
pub enum status {
    Var0(statusVar0),
}
impl status {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = statusVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:135:1"]
#[derive(Clone, Debug)]
pub struct srcREGVar0 {
    fREGLoc: fREGLoc,
}
impl srcREGVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { fREGLoc } = self;
        fREGLoc.display_extend(display, context);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let fREGLoc = if let Some((len, table)) = fREGLoc::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((fREGLoc), (), block_len))
        };
        let ((mut fREGLoc), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { fREGLoc }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:138:1"]
#[derive(Clone, Debug)]
pub struct srcREGVar1 {}
impl srcREGVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PCL")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table srcREG"]
#[derive(Clone, Debug)]
pub enum srcREG {
    Var0(srcREGVar0),
    Var1(srcREGVar1),
}
impl srcREG {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
            Self::Var1(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = srcREGVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        if let Some((inst_next, parsed)) = srcREGVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:165:1"]
#[derive(Clone, Debug)]
pub struct DVar0 {}
impl DVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("f")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.d() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:164:1"]
#[derive(Clone, Debug)]
pub struct DVar1 {}
impl DVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("w")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.d() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table D"]
#[derive(Clone, Debug)]
pub enum D {
    Var0(DVar0),
    Var1(DVar1),
}
impl D {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
            Self::Var1(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = DVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        if let Some((inst_next, parsed)) = DVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:181:1"]
#[derive(Clone, Debug)]
pub struct imm8Var0 {
    k8: u8,
}
impl imm8Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { k8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Unsigned(true, u64::try_from(*k8).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            let k8 = token_parser.k8();
            *context = context_current;
            Some(((), (k8), block_len))
        };
        let ((), (k8), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { k8 }))
    }
}
#[doc = "Table imm8"]
#[derive(Clone, Debug)]
pub enum imm8 {
    Var0(imm8Var0),
}
impl imm8 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = imm8Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12c5xx.slaspec:22:1"]
#[derive(Clone, Debug)]
pub struct trisREGVar0 {}
impl trisREGVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("6")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table trisREG"]
#[derive(Clone, Debug)]
pub enum trisREG {
    Var0(trisREGVar0),
}
impl trisREG {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = trisREGVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:178:1"]
#[derive(Clone, Debug)]
pub struct skipInstVar0 {
    inst_skip: i64,
}
impl skipInstVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { inst_skip } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *inst_skip)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { inst_skip } = self;
        *inst_skip = (i64::try_from(inst_next).unwrap() + (1u64 as i64));
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            let op12 = token_parser.op12();
            *context = context_current;
            Some(((), (op12), block_len))
        };
        let ((), (op12), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_skip = 0i64;
        *context = context_current;
        Some((inst_len, Self { inst_skip }))
    }
}
#[doc = "Table skipInst"]
#[derive(Clone, Debug)]
pub enum skipInst {
    Var0(skipInstVar0),
}
impl skipInst {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = skipInstVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:187:1"]
#[derive(Clone, Debug)]
pub struct pclVar0 {}
impl pclVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("PC")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.f5() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table pcl"]
#[derive(Clone, Debug)]
pub enum pcl {
    Var0(pclVar0),
}
impl pcl {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = pclVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:149:1"]
#[derive(Clone, Debug)]
pub struct destREGVar0 {}
impl destREGVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("1")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.d() != (1u64 as i64) as u8 {
                return None;
            }
            let srcREG = if let Some((len, table)) = srcREG::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u16;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((srcREG), (), block_len))
        };
        let ((mut srcREG), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:148:1"]
#[derive(Clone, Debug)]
pub struct destREGVar1 {}
impl destREGVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("0")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            if token_parser.d() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (), block_len))
        };
        let ((), (), block_len) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table destREG"]
#[derive(Clone, Debug)]
pub enum destREG {
    Var0(destREGVar0),
    Var1(destREGVar1),
}
impl destREG {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
            Self::Var1(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var1(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = destREGVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        if let Some((inst_next, parsed)) = destREGVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var1(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:168:1"]
#[derive(Clone, Debug)]
pub struct absAddr8Var0 {
    k8: u8,
}
impl absAddr8Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { k8 } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Unsigned(true, u64::try_from(*k8).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            let k8 = token_parser.k8();
            *context = context_current;
            Some(((), (k8), block_len))
        };
        let ((), (k8), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { k8 }))
    }
}
#[doc = "Table absAddr8"]
#[derive(Clone, Debug)]
pub enum absAddr8 {
    Var0(absAddr8Var0),
}
impl absAddr8 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = absAddr8Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc:172:1"]
#[derive(Clone, Debug)]
pub struct absAddr9Var0 {
    k9: u16,
}
impl absAddr9Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { k9 } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Unsigned(true, u64::try_from(*k9).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u16,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u16;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u16;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u16;
            let k9 = token_parser.k9();
            *context = context_current;
            Some(((), (k9), block_len))
        };
        let ((), (k9), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { k9 }))
    }
}
#[doc = "Table absAddr9"]
#[derive(Clone, Debug)]
pub enum absAddr9 {
    Var0(absAddr9Var0),
}
impl absAddr9 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.display_extend(display, context),
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u16,
        inst_next: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
        match self {
            Self::Var0(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u16,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u16, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = absAddr9Var0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        None
    }
}
