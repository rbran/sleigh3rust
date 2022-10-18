pub fn meaning_94590151166160(value: usize) -> DisplayElement {
    match value {
        0usize => DisplayElement::Register(Register::r0),
        1usize => DisplayElement::Register(Register::r1),
        2usize => DisplayElement::Register(Register::r2),
        3usize => DisplayElement::Register(Register::r3),
        4usize => DisplayElement::Register(Register::r4),
        5usize => DisplayElement::Register(Register::r5),
        6usize => DisplayElement::Register(Register::r6),
        7usize => DisplayElement::Register(Register::r7),
        8usize => DisplayElement::Register(Register::r8),
        9usize => DisplayElement::Register(Register::r9),
        10usize => DisplayElement::Register(Register::r10),
        11usize => DisplayElement::Register(Register::r11),
        12usize => DisplayElement::Register(Register::r12),
        13usize => DisplayElement::Register(Register::sp),
        14usize => DisplayElement::Register(Register::lr),
        15usize => DisplayElement::Register(Register::pc),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub struct TokenParser16([u8; 2usize]);
impl TokenParser16 {
    pub fn new(data: &[u8]) -> Option<Self> {
        <[u8; 2usize]>::try_from(data).ok().map(Self)
    }
    pub fn imm1214(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 12u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn op0003(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn rs(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn op0007(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 255u128 as u16;
        raw_value as u8
    }
    pub fn op1515(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 15u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn rd(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn simm0010(&self) -> i16 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 2047u128 as u16;
        let unsigned = raw_value & 1023u128 as u16;
        if raw_value & 1024u128 as u16 != 0 {
            (!1023u128 as u16 | unsigned) as i16
        } else {
            unsigned as i16
        }
    }
    pub fn rt(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn op0303(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 3u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn op0811(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn cc0911(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 9u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn op1415(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 14u64 as u16;
        raw_value &= 3u128 as u16;
        raw_value as u8
    }
    pub fn imm0007(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 255u128 as u16;
        raw_value as u8
    }
    pub fn simm0007(&self) -> i8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 255u128 as u16;
        let unsigned = raw_value & 127u128 as u16;
        if raw_value & 128u128 as u16 != 0 {
            (!127u128 as u16 | unsigned) as i8
        } else {
            unsigned as i8
        }
    }
    pub fn simm1213(&self) -> i8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 12u64 as u16;
        raw_value &= 3u128 as u16;
        let unsigned = raw_value & 1u128 as u16;
        if raw_value & 2u128 as u16 != 0 {
            (!1u128 as u16 | unsigned) as i8
        } else {
            unsigned as i8
        }
    }
    pub fn imm0003(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn cc0002(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn simm0003(&self) -> i8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 15u128 as u16;
        let unsigned = raw_value & 7u128 as u16;
        if raw_value & 8u128 as u16 != 0 {
            (!7u128 as u16 | unsigned) as i8
        } else {
            unsigned as i8
        }
    }
    pub fn op1215(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 12u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn op1111(&self) -> u8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 11u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn simm0411(&self) -> i8 {
        let mut raw_value = u16::from_le_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 255u128 as u16;
        let unsigned = raw_value & 127u128 as u16;
        if raw_value & 128u128 as u16 != 0 {
            (!127u128 as u16 | unsigned) as i8
        } else {
            unsigned as i8
        }
    }
}
pub trait GlobalSetTrait {}
pub trait ContextTrait {}
#[derive(Debug, Clone, Copy)]
pub struct Context {}
impl ContextTrait for Context {}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    r10,
    Z,
    N,
    lr,
    r11l,
    r6l,
    r4h,
    lrh,
    r0,
    r8h,
    r11h,
    r12,
    r1l,
    r0l,
    r11,
    r5h,
    r8l,
    r3h,
    r2,
    r8,
    r9,
    spl,
    lrl,
    r5l,
    V,
    r4,
    r2l,
    r7h,
    r3l,
    r9l,
    r12l,
    r12h,
    r3,
    r0h,
    pcl,
    r10l,
    r10h,
    sph,
    r2h,
    r7l,
    r4l,
    r1,
    r1h,
    C,
    pc,
    r9h,
    r6,
    pch,
    r6h,
    sp,
    r5,
    r7,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::r10 => write!(f, "r10"),
            Self::Z => write!(f, "Z"),
            Self::N => write!(f, "N"),
            Self::lr => write!(f, "lr"),
            Self::r11l => write!(f, "r11l"),
            Self::r6l => write!(f, "r6l"),
            Self::r4h => write!(f, "r4h"),
            Self::lrh => write!(f, "lrh"),
            Self::r0 => write!(f, "r0"),
            Self::r8h => write!(f, "r8h"),
            Self::r11h => write!(f, "r11h"),
            Self::r12 => write!(f, "r12"),
            Self::r1l => write!(f, "r1l"),
            Self::r0l => write!(f, "r0l"),
            Self::r11 => write!(f, "r11"),
            Self::r5h => write!(f, "r5h"),
            Self::r8l => write!(f, "r8l"),
            Self::r3h => write!(f, "r3h"),
            Self::r2 => write!(f, "r2"),
            Self::r8 => write!(f, "r8"),
            Self::r9 => write!(f, "r9"),
            Self::spl => write!(f, "spl"),
            Self::lrl => write!(f, "lrl"),
            Self::r5l => write!(f, "r5l"),
            Self::V => write!(f, "V"),
            Self::r4 => write!(f, "r4"),
            Self::r2l => write!(f, "r2l"),
            Self::r7h => write!(f, "r7h"),
            Self::r3l => write!(f, "r3l"),
            Self::r9l => write!(f, "r9l"),
            Self::r12l => write!(f, "r12l"),
            Self::r12h => write!(f, "r12h"),
            Self::r3 => write!(f, "r3"),
            Self::r0h => write!(f, "r0h"),
            Self::pcl => write!(f, "pcl"),
            Self::r10l => write!(f, "r10l"),
            Self::r10h => write!(f, "r10h"),
            Self::sph => write!(f, "sph"),
            Self::r2h => write!(f, "r2h"),
            Self::r7l => write!(f, "r7l"),
            Self::r4l => write!(f, "r4l"),
            Self::r1 => write!(f, "r1"),
            Self::r1h => write!(f, "r1h"),
            Self::C => write!(f, "C"),
            Self::pc => write!(f, "pc"),
            Self::r9h => write!(f, "r9h"),
            Self::r6 => write!(f, "r6"),
            Self::pch => write!(f, "pch"),
            Self::r6h => write!(f, "r6h"),
            Self::sp => write!(f, "sp"),
            Self::r5 => write!(f, "r5"),
            Self::r7 => write!(f, "r7"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:114:1"]
#[derive(Clone, Debug)]
pub struct RTVar0 {
    rt: u8,
}
impl RTVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("["),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
            DisplayElement::Literal("]"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rt = token_parser.rt();
            *context = context_current;
            Some(((), (rt), block_len))
        };
        let ((), (rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { rt }))
    }
}
#[doc = "Table RT"]
#[derive(Clone, Debug)]
pub enum RT {
    Var0(RTVar0),
}
impl RT {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = RTVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:107:1"]
#[derive(Clone, Debug)]
pub struct Imm11Var0 {
    computed: i64,
}
impl Imm11Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { computed } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Signed(true, *computed),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let imm1214 = token_parser.imm1214();
            let imm0007 = token_parser.imm0007();
            *context = context_current;
            Some(((), (imm0007, imm1214), block_len))
        };
        let ((), (imm0007, imm1214), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut computed = 0i64;
        computed = ((i64::try_from(imm1214).unwrap() << (8u64 as i64))
            | i64::try_from(imm0007).unwrap());
        let computed = 0i64;
        *context = context_current;
        Some((inst_len, Self { computed }))
    }
}
#[doc = "Table Imm11"]
#[derive(Clone, Debug)]
pub enum Imm11 {
    Var0(Imm11Var0),
}
impl Imm11 {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = Imm11Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:106:1"]
#[derive(Clone, Debug)]
pub struct Imm4Var0 {
    imm0003: u8,
}
impl Imm4Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { imm0003 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Unsigned(true, u64::try_from(*imm0003).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let imm0003 = token_parser.imm0003();
            *context = context_current;
            Some(((), (imm0003), block_len))
        };
        let ((), (imm0003), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { imm0003 }))
    }
}
#[doc = "Table Imm4"]
#[derive(Clone, Debug)]
pub enum Imm4 {
    Var0(Imm4Var0),
}
impl Imm4 {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = Imm4Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:104:1"]
#[derive(Clone, Debug)]
pub struct Simm10Var0 {
    computed: i64,
}
impl Simm10Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { computed } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Signed(true, *computed),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let simm1213 = token_parser.simm1213();
            let imm0007 = token_parser.imm0007();
            *context = context_current;
            Some(((), (simm1213, imm0007), block_len))
        };
        let ((), (simm1213, imm0007), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut computed = 0i64;
        computed = ((i64::try_from(simm1213).unwrap() << (8u64 as i64))
            | i64::try_from(imm0007).unwrap());
        let computed = 0i64;
        *context = context_current;
        Some((inst_len, Self { computed }))
    }
}
#[doc = "Table Simm10"]
#[derive(Clone, Debug)]
pub enum Simm10 {
    Var0(Simm10Var0),
}
impl Simm10 {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = Simm10Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:110:1"]
#[derive(Clone, Debug)]
pub struct Rel82Var0 {
    addr: i64,
}
impl Rel82Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { addr } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *addr)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let simm0411 = token_parser.simm0411();
            *context = context_current;
            Some(((), (simm0411), block_len))
        };
        let ((), (simm0411), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut addr = 0i64;
        addr = (i64::try_from(inst_start).unwrap()
            + i64::try_from(simm0411).unwrap());
        let addr = 0i64;
        *context = context_current;
        Some((inst_len, Self { addr }))
    }
}
#[doc = "Table Rel82"]
#[derive(Clone, Debug)]
pub enum Rel82 {
    Var0(Rel82Var0),
}
impl Rel82 {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = Rel82Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:111:1"]
#[derive(Clone, Debug)]
pub struct Rel11Var0 {
    addr: i64,
}
impl Rel11Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { addr } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *addr)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let simm0010 = token_parser.simm0010();
            *context = context_current;
            Some(((), (simm0010), block_len))
        };
        let ((), (simm0010), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut addr = 0i64;
        addr = (i64::try_from(inst_start).unwrap()
            + i64::try_from(simm0010).unwrap());
        let addr = 0i64;
        *context = context_current;
        Some((inst_len, Self { addr }))
    }
}
#[doc = "Table Rel11"]
#[derive(Clone, Debug)]
pub enum Rel11 {
    Var0(Rel11Var0),
}
impl Rel11 {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = Rel11Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:125:1"]
#[derive(Clone, Debug)]
pub struct CONDVar0 {
    CC: CC,
}
impl CONDVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { CC } = self;
        CC.display_extend(display, context);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let CC = if let Some((len, table)) =
                CC::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((CC), (), block_len))
        };
        let ((mut CC), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { CC }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:126:1"]
#[derive(Clone, Debug)]
pub struct CONDVar1 {
    CC: CC,
}
impl CONDVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { CC } = self;
        CC.display_extend(display, context);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let CC = if let Some((len, table)) =
                CC::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            if token_parser.cc0002() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((CC), (), block_len))
        };
        let ((mut CC), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { CC }))
    }
}
#[doc = "Table COND"]
#[derive(Clone, Debug)]
pub enum COND {
    Var0(CONDVar0),
    Var1(CONDVar1),
}
impl COND {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = CONDVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        if let Some((inst_next, parsed)) = CONDVar1::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:103:1"]
#[derive(Clone, Debug)]
pub struct Simm4Var0 {
    simm0003: i8,
}
impl Simm4Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { simm0003 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Signed(true, i64::try_from(*simm0003).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let simm0003 = token_parser.simm0003();
            *context = context_current;
            Some(((), (simm0003), block_len))
        };
        let ((), (simm0003), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { simm0003 }))
    }
}
#[doc = "Table Simm4"]
#[derive(Clone, Debug)]
pub enum Simm4 {
    Var0(Simm4Var0),
}
impl Simm4 {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = Simm4Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:201:1"]
#[derive(Clone, Debug)]
pub struct instructionVar0 {
    Rel82: Rel82,
    COND: COND,
}
impl instructionVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Rel82, COND } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("brds")];
        display.extend_from_slice(&extend);
        COND.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        Rel82.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (14u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0303() != (1u64 as i64) as u8 {
                return None;
            }
            let COND = if let Some((len, table)) = COND::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            let Rel82 = if let Some((len, table)) = Rel82::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Rel82, COND), (), block_len))
        };
        let ((mut Rel82, mut COND), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Rel82.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        COND.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Rel82, COND }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:200:1"]
#[derive(Clone, Debug)]
pub struct instructionVar1 {
    COND: COND,
    Rel82: Rel82,
}
impl instructionVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { COND, Rel82 } = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("br")];
        display.extend_from_slice(&extend);
        COND.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        Rel82.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (14u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0303() != (0u64 as i64) as u8 {
                return None;
            }
            let COND = if let Some((len, table)) = COND::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            let Rel82 = if let Some((len, table)) = Rel82::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((COND, Rel82), (), block_len))
        };
        let ((mut COND, mut Rel82), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        COND.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        Rel82.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { COND, Rel82 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:223:1"]
#[derive(Clone, Debug)]
pub struct instructionVar2 {
    Rel8: Rel8,
}
impl instructionVar2 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Rel8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("user_five"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        Rel8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (10u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (5u64 as i64) as u8 {
                return None;
            }
            let Rel8 = if let Some((len, table)) = Rel8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Rel8), (), block_len))
        };
        let ((mut Rel8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Rel8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:212:1"]
#[derive(Clone, Debug)]
pub struct instructionVar3 {
    Rel8: Rel8,
}
impl instructionVar3 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Rel8 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("callds"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        Rel8.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (5u64 as i64) as u8 {
                return None;
            }
            let Rel8 = if let Some((len, table)) = Rel8::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Rel8), (), block_len))
        };
        let ((mut Rel8), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Rel8.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:214:1"]
#[derive(Clone, Debug)]
pub struct instructionVar4 {
    Rel11: Rel11,
}
impl instructionVar4 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Rel11 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("call"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        Rel11.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op1111() != (1u64 as i64) as u8 {
                return None;
            }
            let Rel11 = if let Some((len, table)) = Rel11::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Rel11), (), block_len))
        };
        let ((mut Rel11), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Rel11.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Rel11 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:152:1"]
#[derive(Clone, Debug)]
pub struct instructionVar5 {
    Simm10: Simm10,
    rd: u8,
}
impl instructionVar5 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Simm10, rd } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("simm"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rd).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Simm10.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rd = token_parser.rd();
            if token_parser.op1415() != (2u64 as i64) as u8 {
                return None;
            }
            let Simm10 = if let Some((len, table)) = Simm10::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Simm10), (rd), block_len))
        };
        let ((mut Simm10), (rd), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Simm10.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Simm10, rd }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:151:1"]
#[derive(Clone, Debug)]
pub struct instructionVar6 {
    Imm11: Imm11,
    rd: u8,
}
impl instructionVar6 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Imm11, rd } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("imm"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rd).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Imm11.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rd = token_parser.rd();
            if token_parser.op1515() != (0u64 as i64) as u8 {
                return None;
            }
            let Imm11 = if let Some((len, table)) = Imm11::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Imm11), (rd), block_len))
        };
        let ((mut Imm11), (rd), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Imm11.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Imm11, rd }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:224:1"]
#[derive(Clone, Debug)]
pub struct instructionVar7 {
    rs: u8,
}
impl instructionVar7 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("user_six"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (10u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (6u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:220:1"]
#[derive(Clone, Debug)]
pub struct instructionVar8 {
    rs: u8,
}
impl instructionVar8 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("user_two"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (10u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (2u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:219:1"]
#[derive(Clone, Debug)]
pub struct instructionVar9 {
    rs: u8,
}
impl instructionVar9 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("user_one"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (10u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (1u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:213:1"]
#[derive(Clone, Debug)]
pub struct instructionVar10 {
    rs: u8,
}
impl instructionVar10 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("call"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (6u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:209:1"]
#[derive(Clone, Debug)]
pub struct instructionVar11 {
    rs: u8,
}
impl instructionVar11 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("pop"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (3u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:208:1"]
#[derive(Clone, Debug)]
pub struct instructionVar12 {
    rs: u8,
}
impl instructionVar12 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("push"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (2u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:194:1"]
#[derive(Clone, Debug)]
pub struct instructionVar13 {
    rs: u8,
}
impl instructionVar13 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("neg"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (14u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:193:1"]
#[derive(Clone, Debug)]
pub struct instructionVar14 {
    rs: u8,
}
impl instructionVar14 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("inv"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (14u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0003() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:217:1"]
#[derive(Clone, Debug)]
pub struct instructionVar15 {
    COND: COND,
    rs: u8,
}
impl instructionVar15 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { COND, rs } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("call")];
        display.extend_from_slice(&extend);
        COND.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (6u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0303() != (1u64 as i64) as u8 {
                return None;
            }
            let COND = if let Some((len, table)) = COND::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((COND), (rs), block_len))
        };
        let ((mut COND), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        COND.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:203:1"]
#[derive(Clone, Debug)]
pub struct instructionVar16 {
    COND: COND,
    rs: u8,
}
impl instructionVar16 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { COND, rs } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("brds")];
        display.extend_from_slice(&extend);
        COND.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (1u64 as i64) as u8 {
                return None;
            }
            let COND = if let Some((len, table)) = COND::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            if token_parser.op0303() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((COND), (rs), block_len))
        };
        let ((mut COND), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        COND.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:202:1"]
#[derive(Clone, Debug)]
pub struct instructionVar17 {
    COND: COND,
    rs: u8,
}
impl instructionVar17 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { COND, rs } = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("br")];
        display.extend_from_slice(&extend);
        COND.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (0u64 as i64) as u8 {
                return None;
            }
            let COND = if let Some((len, table)) = COND::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            if token_parser.op0303() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((COND), (rs), block_len))
        };
        let ((mut COND), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        COND.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:222:1"]
#[derive(Clone, Debug)]
pub struct instructionVar18 {
    rt: u8,
    rs: u8,
}
impl instructionVar18 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("user_four"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (10u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:198:1"]
#[derive(Clone, Debug)]
pub struct instructionVar19 {
    rt: u8,
    rs: u8,
}
impl instructionVar19 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mov"),
            DisplayElement::Literal("   "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (15u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:188:1"]
#[derive(Clone, Debug)]
pub struct instructionVar20 {
    rs: u8,
    rt: u8,
}
impl instructionVar20 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("saa"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (8u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:177:1"]
#[derive(Clone, Debug)]
pub struct instructionVar21 {
    rs: u8,
    rt: u8,
}
impl instructionVar21 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lsl"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (5u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:176:1"]
#[derive(Clone, Debug)]
pub struct instructionVar22 {
    rt: u8,
    rs: u8,
}
impl instructionVar22 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("asr"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:175:1"]
#[derive(Clone, Debug)]
pub struct instructionVar23 {
    rt: u8,
    rs: u8,
}
impl instructionVar23 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("lsr"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:174:1"]
#[derive(Clone, Debug)]
pub struct instructionVar24 {
    rs: u8,
    rt: u8,
}
impl instructionVar24 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("xor"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:173:1"]
#[derive(Clone, Debug)]
pub struct instructionVar25 {
    rt: u8,
    rs: u8,
}
impl instructionVar25 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("or"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:172:1"]
#[derive(Clone, Debug)]
pub struct instructionVar26 {
    rs: u8,
    rt: u8,
}
impl instructionVar26 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("and"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:161:1"]
#[derive(Clone, Debug)]
pub struct instructionVar27 {
    rs: u8,
    rt: u8,
}
impl instructionVar27 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("ucmp"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:160:1"]
#[derive(Clone, Debug)]
pub struct instructionVar28 {
    rs: u8,
    rt: u8,
}
impl instructionVar28 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:159:1"]
#[derive(Clone, Debug)]
pub struct instructionVar29 {
    rt: u8,
    rs: u8,
}
impl instructionVar29 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mod"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (5u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:158:1"]
#[derive(Clone, Debug)]
pub struct instructionVar30 {
    rt: u8,
    rs: u8,
}
impl instructionVar30 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("div"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:157:1"]
#[derive(Clone, Debug)]
pub struct instructionVar31 {
    rt: u8,
    rs: u8,
}
impl instructionVar31 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rt, rs } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("mul"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rt, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:156:1"]
#[derive(Clone, Debug)]
pub struct instructionVar32 {
    rs: u8,
    rt: u8,
}
impl instructionVar32 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("rsub"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rs, rt), block_len))
        };
        let ((), (rs, rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:155:1"]
#[derive(Clone, Debug)]
pub struct instructionVar33 {
    rs: u8,
    rt: u8,
}
impl instructionVar33 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("sub"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:154:1"]
#[derive(Clone, Debug)]
pub struct instructionVar34 {
    rs: u8,
    rt: u8,
}
impl instructionVar34 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs, rt } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("add"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            let rt = token_parser.rt();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (rt, rs), block_len))
        };
        let ((), (rt, rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:226:1"]
#[derive(Clone, Debug)]
pub struct instructionVar35 {}
impl instructionVar35 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("unimpl")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (10u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (8u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0007() != (0u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:221:1"]
#[derive(Clone, Debug)]
pub struct instructionVar36 {}
impl instructionVar36 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("user_three")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (10u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (3u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0007() != (0u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:211:1"]
#[derive(Clone, Debug)]
pub struct instructionVar37 {}
impl instructionVar37 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ret")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.op1215() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (4u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0007() != (0u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:197:1"]
#[derive(Clone, Debug)]
pub struct instructionVar38 {
    RS: RS,
    rt: u8,
}
impl instructionVar38 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { RS, rt } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("store"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        RS.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(", "),
            meaning_94590151166160(usize::try_from(*rt).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rt = token_parser.rt();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (7u64 as i64) as u8 {
                return None;
            }
            let RS = if let Some((len, table)) =
                RS::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((RS), (rt), block_len))
        };
        let ((mut RS), (rt), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        RS.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { RS, rt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:196:1"]
#[derive(Clone, Debug)]
pub struct instructionVar39 {
    RT: RT,
    rs: u8,
}
impl instructionVar39 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { RT, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("load"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        RT.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (6u64 as i64) as u8 {
                return None;
            }
            let RT = if let Some((len, table)) =
                RT::parse(tokens, &mut context_current, inst_start, global_set)
            {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((RT), (rs), block_len))
        };
        let ((mut RT), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        RT.disassembly(&mut context_current, inst_start, inst_next, global_set);
        *context = context_current;
        Some((inst_len, Self { RT, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:192:1"]
#[derive(Clone, Debug)]
pub struct instructionVar40 {
    Imm4: Imm4,
    rs: u8,
}
impl instructionVar40 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Imm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("lsl"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Imm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (13u64 as i64) as u8 {
                return None;
            }
            let Imm4 = if let Some((len, table)) = Imm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Imm4), (rs), block_len))
        };
        let ((mut Imm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Imm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:191:1"]
#[derive(Clone, Debug)]
pub struct instructionVar41 {
    Imm4: Imm4,
    rs: u8,
}
impl instructionVar41 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Imm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("asr"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Imm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (12u64 as i64) as u8 {
                return None;
            }
            let Imm4 = if let Some((len, table)) = Imm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Imm4), (rs), block_len))
        };
        let ((mut Imm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Imm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:190:1"]
#[derive(Clone, Debug)]
pub struct instructionVar42 {
    Imm4: Imm4,
    rs: u8,
}
impl instructionVar42 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Imm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("lsr"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Imm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (13u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (11u64 as i64) as u8 {
                return None;
            }
            let Imm4 = if let Some((len, table)) = Imm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Imm4), (rs), block_len))
        };
        let ((mut Imm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Imm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:170:1"]
#[derive(Clone, Debug)]
pub struct instructionVar43 {
    Imm4: Imm4,
    rs: u8,
}
impl instructionVar43 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Imm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("ucmp"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Imm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (15u64 as i64) as u8 {
                return None;
            }
            let Imm4 = if let Some((len, table)) = Imm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Imm4), (rs), block_len))
        };
        let ((mut Imm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Imm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:169:1"]
#[derive(Clone, Debug)]
pub struct instructionVar44 {
    Simm4: Simm4,
    rs: u8,
}
impl instructionVar44 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Simm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Simm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (14u64 as i64) as u8 {
                return None;
            }
            let Simm4 = if let Some((len, table)) = Simm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Simm4), (rs), block_len))
        };
        let ((mut Simm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Simm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:168:1"]
#[derive(Clone, Debug)]
pub struct instructionVar45 {
    Imm4: Imm4,
    rs: u8,
}
impl instructionVar45 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Imm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mod"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Imm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (13u64 as i64) as u8 {
                return None;
            }
            let Imm4 = if let Some((len, table)) = Imm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Imm4), (rs), block_len))
        };
        let ((mut Imm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Imm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:167:1"]
#[derive(Clone, Debug)]
pub struct instructionVar46 {
    Simm4: Simm4,
    rs: u8,
}
impl instructionVar46 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Simm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("div"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Simm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (12u64 as i64) as u8 {
                return None;
            }
            let Simm4 = if let Some((len, table)) = Simm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Simm4), (rs), block_len))
        };
        let ((mut Simm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Simm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:166:1"]
#[derive(Clone, Debug)]
pub struct instructionVar47 {
    Simm4: Simm4,
    rs: u8,
}
impl instructionVar47 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Simm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mul"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Simm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (11u64 as i64) as u8 {
                return None;
            }
            let Simm4 = if let Some((len, table)) = Simm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Simm4), (rs), block_len))
        };
        let ((mut Simm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Simm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:165:1"]
#[derive(Clone, Debug)]
pub struct instructionVar48 {
    Simm4: Simm4,
    rs: u8,
}
impl instructionVar48 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Simm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("rsub"),
            DisplayElement::Literal(" "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Simm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (10u64 as i64) as u8 {
                return None;
            }
            let Simm4 = if let Some((len, table)) = Simm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Simm4), (rs), block_len))
        };
        let ((mut Simm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Simm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:164:1"]
#[derive(Clone, Debug)]
pub struct instructionVar49 {
    Simm4: Simm4,
    rs: u8,
}
impl instructionVar49 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Simm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("sub"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Simm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (9u64 as i64) as u8 {
                return None;
            }
            let Simm4 = if let Some((len, table)) = Simm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Simm4), (rs), block_len))
        };
        let ((mut Simm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Simm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:163:1"]
#[derive(Clone, Debug)]
pub struct instructionVar50 {
    Simm4: Simm4,
    rs: u8,
}
impl instructionVar50 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { Simm4, rs } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("add"),
            DisplayElement::Literal("  "),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        Simm4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            if token_parser.op1215() != (12u64 as i64) as u8 {
                return None;
            }
            if token_parser.op0811() != (8u64 as i64) as u8 {
                return None;
            }
            let Simm4 = if let Some((len, table)) = Simm4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u64;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((Simm4), (rs), block_len))
        };
        let ((mut Simm4), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        Simm4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { Simm4, rs }))
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:113:1"]
#[derive(Clone, Debug)]
pub struct RSVar0 {
    rs: u8,
}
impl RSVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { rs } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("["),
            meaning_94590151166160(usize::try_from(*rs).unwrap()),
            DisplayElement::Literal("]"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let rs = token_parser.rs();
            *context = context_current;
            Some(((), (rs), block_len))
        };
        let ((), (rs), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { rs }))
    }
}
#[doc = "Table RS"]
#[derive(Clone, Debug)]
pub enum RS {
    Var0(RSVar0),
}
impl RS {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = RSVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:109:1"]
#[derive(Clone, Debug)]
pub struct Rel8Var0 {
    addr: i64,
}
impl Rel8Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { addr } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *addr)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            let simm0007 = token_parser.simm0007();
            *context = context_current;
            Some(((), (simm0007), block_len))
        };
        let ((), (simm0007), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut addr = 0i64;
        addr = (i64::try_from(inst_start).unwrap()
            + i64::try_from(simm0007).unwrap());
        let addr = 0i64;
        *context = context_current;
        Some((inst_len, Self { addr }))
    }
}
#[doc = "Table Rel8"]
#[derive(Clone, Debug)]
pub enum Rel8 {
    Var0(Rel8Var0),
}
impl Rel8 {
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
        inst_start: u64,
        inst_next: u64,
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = Rel8Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:123:1"]
#[derive(Clone, Debug)]
pub struct CCVar0 {}
impl CCVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal(""), DisplayElement::Literal("")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (7u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:122:1"]
#[derive(Clone, Debug)]
pub struct CCVar1 {}
impl CCVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("vs")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (6u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:121:1"]
#[derive(Clone, Debug)]
pub struct CCVar2 {}
impl CCVar2 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("mi")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (5u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:120:1"]
#[derive(Clone, Debug)]
pub struct CCVar3 {}
impl CCVar3 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("lo")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (4u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:119:1"]
#[derive(Clone, Debug)]
pub struct CCVar4 {}
impl CCVar4 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("le")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (3u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:118:1"]
#[derive(Clone, Debug)]
pub struct CCVar5 {}
impl CCVar5 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("lt")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (2u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:117:1"]
#[derive(Clone, Debug)]
pub struct CCVar6 {}
impl CCVar6 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("ne")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (1u64 as i64) as u8 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc:116:1"]
#[derive(Clone, Debug)]
pub struct CCVar7 {}
impl CCVar7 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("eq")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u64,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u64;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u64;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u64;
            if token_parser.cc0002() != (0u64 as i64) as u8 {
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
#[doc = "Table CC"]
#[derive(Clone, Debug)]
pub enum CC {
    Var0(CCVar0),
    Var1(CCVar1),
    Var2(CCVar2),
    Var3(CCVar3),
    Var4(CCVar4),
    Var5(CCVar5),
    Var6(CCVar6),
    Var7(CCVar7),
}
impl CC {
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
        }
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u64,
        inst_next: u64,
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
            Self::Var6(x) => x.disassembly(
                context_param,
                inst_start,
                inst_next,
                global_set_param,
            ),
            Self::Var7(x) => x.disassembly(
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
        inst_start: u64,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u64, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = CCVar0::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var0(parsed)));
        }
        if let Some((inst_next, parsed)) = CCVar1::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var1(parsed)));
        }
        if let Some((inst_next, parsed)) = CCVar2::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var2(parsed)));
        }
        if let Some((inst_next, parsed)) = CCVar3::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var3(parsed)));
        }
        if let Some((inst_next, parsed)) = CCVar4::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var4(parsed)));
        }
        if let Some((inst_next, parsed)) = CCVar5::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var5(parsed)));
        }
        if let Some((inst_next, parsed)) = CCVar6::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var6(parsed)));
        }
        if let Some((inst_next, parsed)) = CCVar7::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var7(parsed)));
        }
        None
    }
}
