pub fn meaning_94590056407392(value: usize) -> DisplayElement {
    match value {
        0usize => DisplayElement::Register(Register::fv0),
        1usize => DisplayElement::Register(Register::fv4),
        2usize => DisplayElement::Register(Register::fv8),
        3usize => DisplayElement::Register(Register::fv12),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub fn meaning_94590034762480(value: usize) -> DisplayElement {
    match value {
        0usize => DisplayElement::Register(Register::R0_BANK),
        1usize => DisplayElement::Register(Register::R1_BANK),
        2usize => DisplayElement::Register(Register::R2_BANK),
        3usize => DisplayElement::Register(Register::R3_BANK),
        4usize => DisplayElement::Register(Register::R4_BANK),
        5usize => DisplayElement::Register(Register::R5_BANK),
        6usize => DisplayElement::Register(Register::R6_BANK),
        7usize => DisplayElement::Register(Register::R7_BANK),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub fn meaning_94590182964544(value: usize) -> DisplayElement {
    match value {
        0usize => DisplayElement::Register(Register::fr0),
        1usize => DisplayElement::Register(Register::fr1),
        2usize => DisplayElement::Register(Register::fr2),
        3usize => DisplayElement::Register(Register::fr3),
        4usize => DisplayElement::Register(Register::fr4),
        5usize => DisplayElement::Register(Register::fr5),
        6usize => DisplayElement::Register(Register::fr6),
        7usize => DisplayElement::Register(Register::fr7),
        8usize => DisplayElement::Register(Register::fr8),
        9usize => DisplayElement::Register(Register::fr9),
        10usize => DisplayElement::Register(Register::fr10),
        11usize => DisplayElement::Register(Register::fr11),
        12usize => DisplayElement::Register(Register::fr12),
        13usize => DisplayElement::Register(Register::fr13),
        14usize => DisplayElement::Register(Register::fr14),
        15usize => DisplayElement::Register(Register::fr15),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub fn meaning_94590151099744(value: usize) -> DisplayElement {
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
        13usize => DisplayElement::Register(Register::r13),
        14usize => DisplayElement::Register(Register::r14),
        15usize => DisplayElement::Register(Register::r15),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub fn meaning_94590061630544(value: usize) -> DisplayElement {
    match value {
        0usize => DisplayElement::Register(Register::dr0),
        1usize => DisplayElement::Register(Register::dr2),
        2usize => DisplayElement::Register(Register::dr4),
        3usize => DisplayElement::Register(Register::dr6),
        4usize => DisplayElement::Register(Register::dr8),
        5usize => DisplayElement::Register(Register::dr10),
        6usize => DisplayElement::Register(Register::dr12),
        7usize => DisplayElement::Register(Register::dr14),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub fn meaning_94590141236368(value: usize) -> DisplayElement {
    match value {
        0usize => DisplayElement::Register(Register::dr0),
        1usize => DisplayElement::Register(Register::xd0),
        2usize => DisplayElement::Register(Register::dr2),
        3usize => DisplayElement::Register(Register::xd2),
        4usize => DisplayElement::Register(Register::dr4),
        5usize => DisplayElement::Register(Register::xd4),
        6usize => DisplayElement::Register(Register::dr6),
        7usize => DisplayElement::Register(Register::xd6),
        8usize => DisplayElement::Register(Register::dr8),
        9usize => DisplayElement::Register(Register::xd8),
        10usize => DisplayElement::Register(Register::dr10),
        11usize => DisplayElement::Register(Register::xd10),
        12usize => DisplayElement::Register(Register::dr12),
        13usize => DisplayElement::Register(Register::xd12),
        14usize => DisplayElement::Register(Register::dr14),
        15usize => DisplayElement::Register(Register::xd14),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub fn meaning_94590049788064(value: usize) -> DisplayElement {
    match value {
        0usize => DisplayElement::Register(Register::xd0),
        1usize => DisplayElement::Register(Register::xd2),
        2usize => DisplayElement::Register(Register::xd4),
        3usize => DisplayElement::Register(Register::xd6),
        4usize => DisplayElement::Register(Register::xd8),
        5usize => DisplayElement::Register(Register::xd10),
        6usize => DisplayElement::Register(Register::xd12),
        7usize => DisplayElement::Register(Register::xd14),
        _ => unreachable!("Invalid Attach Value"),
    }
}
pub struct TokenParser16([u8; 2usize]);
impl TokenParser16 {
    pub fn new(data: &[u8]) -> Option<Self> {
        <[u8; 2usize]>::try_from(data).ok().map(Self)
    }
    pub fn OP_7(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 31u128 as u16;
        raw_value as u8
    }
    pub fn XDM_1(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 5u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn FRM_0(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn DRN_1(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 9u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn OP_4(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 255u128 as u16;
        raw_value as u8
    }
    pub fn M_2(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 3u128 as u16;
        raw_value as u8
    }
    pub fn I_0(&self) -> i8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 255u128 as u16;
        let unsigned = raw_value & 127u128 as u16;
        if raw_value & 128u128 as u16 != 0 {
            (!127u128 as u16 | unsigned) as i8
        } else {
            unsigned as i8
        }
    }
    pub fn FVM_2(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 3u128 as u16;
        raw_value as u8
    }
    pub fn XDRN(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn OP_0(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 12u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn OP_10(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn OP_5(&self) -> u16 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 511u128 as u16;
        raw_value as u16
    }
    pub fn FRN_0(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn XDRM(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn OP_6(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn N_0(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn U_0(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 255u128 as u16;
        raw_value as u8
    }
    pub fn U_1(&self) -> u16 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 4095u128 as u16;
        raw_value as u16
    }
    pub fn N_1(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 9u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn OP_1(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn I_2(&self) -> i8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 15u128 as u16;
        let unsigned = raw_value & 7u128 as u16;
        if raw_value & 8u128 as u16 != 0 {
            (!7u128 as u16 | unsigned) as i8
        } else {
            unsigned as i8
        }
    }
    pub fn I_1(&self) -> i16 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 4095u128 as u16;
        let unsigned = raw_value & 2047u128 as u16;
        if raw_value & 2048u128 as u16 != 0 {
            (!2047u128 as u16 | unsigned) as i16
        } else {
            unsigned as i16
        }
    }
    pub fn U_2(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn XDN_1(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 9u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn M_0(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 15u128 as u16;
        raw_value as u8
    }
    pub fn M_1(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 5u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn OP_9(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 7u64 as u16;
        raw_value &= 1u128 as u16;
        raw_value as u8
    }
    pub fn OP_3(&self) -> u16 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 65535u128 as u16;
        raw_value as u16
    }
    pub fn OP_8(&self) -> u16 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 0u64 as u16;
        raw_value &= 1023u128 as u16;
        raw_value as u16
    }
    pub fn FVN_2(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 10u64 as u16;
        raw_value &= 3u128 as u16;
        raw_value as u8
    }
    pub fn N_2(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 10u64 as u16;
        raw_value &= 3u128 as u16;
        raw_value as u8
    }
    pub fn BANK(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 4u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn DRM_1(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 5u64 as u16;
        raw_value &= 7u128 as u16;
        raw_value as u8
    }
    pub fn OP_2(&self) -> u8 {
        let mut raw_value = u16::from_be_bytes(self.0);
        raw_value >>= 8u64 as u16;
        raw_value &= 255u128 as u16;
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
    fr13,
    fr9,
    xf9,
    r4,
    fv0,
    fv12,
    fr1,
    xd6,
    xf14,
    xf10,
    r9,
    xd12,
    dr4,
    r15,
    VBR,
    r5,
    FPSCR_ENABLE,
    fr2,
    fr0,
    fr7,
    FPSCR_SZ,
    fr11,
    fr6,
    SSR,
    R6_BANK,
    RB,
    xd4,
    BL,
    fv8,
    xf6,
    FD,
    fr12,
    r6,
    fr14,
    dr2,
    fr5,
    xf13,
    xf7,
    xd2,
    fr8,
    dr8,
    r11,
    dr6,
    R0_BANK,
    R2_BANK,
    xf4,
    xf11,
    T,
    r1,
    r14,
    PC,
    dr0,
    MACH,
    xd0,
    fr4,
    M,
    IMASK,
    xf3,
    r0,
    r7,
    xf8,
    xf15,
    xd14,
    R3_BANK,
    FPSCR_PR,
    xf2,
    DBR,
    R5_BANK,
    dr12,
    dr14,
    r10,
    fv4,
    FPSCR_RM,
    Q,
    xf0,
    fr3,
    GBR,
    S,
    r13,
    FPUL,
    xf5,
    FPSCR,
    fr15,
    r3,
    xf12,
    dr10,
    MD,
    FPSCR_DN,
    SGR,
    SR,
    R4_BANK,
    FPSCR_FR,
    FPSCR_CAUSE,
    R7_BANK,
    r8,
    xf1,
    FPSCR_FLAG,
    PR,
    MACL,
    xd10,
    r12,
    xd8,
    R1_BANK,
    r2,
    fr10,
    SPC,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::fr13 => write!(f, "fr13"),
            Self::fr9 => write!(f, "fr9"),
            Self::xf9 => write!(f, "xf9"),
            Self::r4 => write!(f, "r4"),
            Self::fv0 => write!(f, "fv0"),
            Self::fv12 => write!(f, "fv12"),
            Self::fr1 => write!(f, "fr1"),
            Self::xd6 => write!(f, "xd6"),
            Self::xf14 => write!(f, "xf14"),
            Self::xf10 => write!(f, "xf10"),
            Self::r9 => write!(f, "r9"),
            Self::xd12 => write!(f, "xd12"),
            Self::dr4 => write!(f, "dr4"),
            Self::r15 => write!(f, "r15"),
            Self::VBR => write!(f, "VBR"),
            Self::r5 => write!(f, "r5"),
            Self::FPSCR_ENABLE => write!(f, "FPSCR_ENABLE"),
            Self::fr2 => write!(f, "fr2"),
            Self::fr0 => write!(f, "fr0"),
            Self::fr7 => write!(f, "fr7"),
            Self::FPSCR_SZ => write!(f, "FPSCR_SZ"),
            Self::fr11 => write!(f, "fr11"),
            Self::fr6 => write!(f, "fr6"),
            Self::SSR => write!(f, "SSR"),
            Self::R6_BANK => write!(f, "R6_BANK"),
            Self::RB => write!(f, "RB"),
            Self::xd4 => write!(f, "xd4"),
            Self::BL => write!(f, "BL"),
            Self::fv8 => write!(f, "fv8"),
            Self::xf6 => write!(f, "xf6"),
            Self::FD => write!(f, "FD"),
            Self::fr12 => write!(f, "fr12"),
            Self::r6 => write!(f, "r6"),
            Self::fr14 => write!(f, "fr14"),
            Self::dr2 => write!(f, "dr2"),
            Self::fr5 => write!(f, "fr5"),
            Self::xf13 => write!(f, "xf13"),
            Self::xf7 => write!(f, "xf7"),
            Self::xd2 => write!(f, "xd2"),
            Self::fr8 => write!(f, "fr8"),
            Self::dr8 => write!(f, "dr8"),
            Self::r11 => write!(f, "r11"),
            Self::dr6 => write!(f, "dr6"),
            Self::R0_BANK => write!(f, "R0_BANK"),
            Self::R2_BANK => write!(f, "R2_BANK"),
            Self::xf4 => write!(f, "xf4"),
            Self::xf11 => write!(f, "xf11"),
            Self::T => write!(f, "T"),
            Self::r1 => write!(f, "r1"),
            Self::r14 => write!(f, "r14"),
            Self::PC => write!(f, "PC"),
            Self::dr0 => write!(f, "dr0"),
            Self::MACH => write!(f, "MACH"),
            Self::xd0 => write!(f, "xd0"),
            Self::fr4 => write!(f, "fr4"),
            Self::M => write!(f, "M"),
            Self::IMASK => write!(f, "IMASK"),
            Self::xf3 => write!(f, "xf3"),
            Self::r0 => write!(f, "r0"),
            Self::r7 => write!(f, "r7"),
            Self::xf8 => write!(f, "xf8"),
            Self::xf15 => write!(f, "xf15"),
            Self::xd14 => write!(f, "xd14"),
            Self::R3_BANK => write!(f, "R3_BANK"),
            Self::FPSCR_PR => write!(f, "FPSCR_PR"),
            Self::xf2 => write!(f, "xf2"),
            Self::DBR => write!(f, "DBR"),
            Self::R5_BANK => write!(f, "R5_BANK"),
            Self::dr12 => write!(f, "dr12"),
            Self::dr14 => write!(f, "dr14"),
            Self::r10 => write!(f, "r10"),
            Self::fv4 => write!(f, "fv4"),
            Self::FPSCR_RM => write!(f, "FPSCR_RM"),
            Self::Q => write!(f, "Q"),
            Self::xf0 => write!(f, "xf0"),
            Self::fr3 => write!(f, "fr3"),
            Self::GBR => write!(f, "GBR"),
            Self::S => write!(f, "S"),
            Self::r13 => write!(f, "r13"),
            Self::FPUL => write!(f, "FPUL"),
            Self::xf5 => write!(f, "xf5"),
            Self::FPSCR => write!(f, "FPSCR"),
            Self::fr15 => write!(f, "fr15"),
            Self::r3 => write!(f, "r3"),
            Self::xf12 => write!(f, "xf12"),
            Self::dr10 => write!(f, "dr10"),
            Self::MD => write!(f, "MD"),
            Self::FPSCR_DN => write!(f, "FPSCR_DN"),
            Self::SGR => write!(f, "SGR"),
            Self::SR => write!(f, "SR"),
            Self::R4_BANK => write!(f, "R4_BANK"),
            Self::FPSCR_FR => write!(f, "FPSCR_FR"),
            Self::FPSCR_CAUSE => write!(f, "FPSCR_CAUSE"),
            Self::R7_BANK => write!(f, "R7_BANK"),
            Self::r8 => write!(f, "r8"),
            Self::xf1 => write!(f, "xf1"),
            Self::FPSCR_FLAG => write!(f, "FPSCR_FLAG"),
            Self::PR => write!(f, "PR"),
            Self::MACL => write!(f, "MACL"),
            Self::xd10 => write!(f, "xd10"),
            Self::r12 => write!(f, "r12"),
            Self::xd8 => write!(f, "xd8"),
            Self::R1_BANK => write!(f, "R1_BANK"),
            Self::r2 => write!(f, "r2"),
            Self::fr10 => write!(f, "fr10"),
            Self::SPC => write!(f, "SPC"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:531:1"]
#[derive(Clone, Debug)]
pub struct fpscr_tVar0 {}
impl fpscr_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("FPSCR")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table fpscr_t"]
#[derive(Clone, Debug)]
pub enum fpscr_t {
    Var0(fpscr_tVar0),
}
impl fpscr_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = fpscr_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:504:1"]
#[derive(Clone, Debug)]
pub struct N_0t_bankVar0 {
    N_0: u8,
}
impl N_0t_bankVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590151099744(usize::try_from(*N_0).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_bank"]
#[derive(Clone, Debug)]
pub enum N_0t_bank {
    Var0(N_0t_bankVar0),
}
impl N_0t_bank {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_bankVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:585:1"]
#[derive(Clone, Debug)]
pub struct M_0t_at_with_r0Var0 {
    M_0: u8,
}
impl M_0t_at_with_r0Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*M_0).unwrap()),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let M_0 = token_parser.M_0();
            *context = context_current;
            Some(((), (M_0), block_len))
        };
        let ((), (M_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { M_0 }))
    }
}
#[doc = "Table M_0t_at_with_r0"]
#[derive(Clone, Debug)]
pub enum M_0t_at_with_r0 {
    Var0(M_0t_at_with_r0Var0),
}
impl M_0t_at_with_r0 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = M_0t_at_with_r0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:446:1"]
#[derive(Clone, Debug)]
pub struct N_0tVar0 {
    N_0: u8,
}
impl N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590151099744(usize::try_from(*N_0).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t"]
#[derive(Clone, Debug)]
pub enum N_0t {
    Var0(N_0tVar0),
}
impl N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:485:1"]
#[derive(Clone, Debug)]
pub struct spc_tVar0 {}
impl spc_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::SPC)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table spc_t"]
#[derive(Clone, Debug)]
pub enum spc_t {
    Var0(spc_tVar0),
}
impl spc_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = spc_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:557:1"]
#[derive(Clone, Debug)]
pub struct N_0t_pr1Var0 {
    N_0: u8,
}
impl N_0t_pr1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::PR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_pr1"]
#[derive(Clone, Debug)]
pub enum N_0t_pr1 {
    Var0(N_0t_pr1Var0),
}
impl N_0t_pr1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_pr1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:561:1"]
#[derive(Clone, Debug)]
pub struct N_0t_fpscr1Var0 {
    N_0: u8,
}
impl N_0t_fpscr1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,fpscr"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_fpscr1"]
#[derive(Clone, Debug)]
pub enum N_0t_fpscr1 {
    Var0(N_0t_fpscr1Var0),
}
impl N_0t_fpscr1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_fpscr1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:569:1"]
#[derive(Clone, Debug)]
pub struct N_0t_atVar0 {
    N_0: u8,
}
impl N_0t_atVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_at"]
#[derive(Clone, Debug)]
pub enum N_0t_at {
    Var0(N_0t_atVar0),
}
impl N_0t_at {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_atVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:495:1"]
#[derive(Clone, Debug)]
pub struct N_0t_vbrVar0 {
    N_0: u8,
}
impl N_0t_vbrVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::VBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_vbr"]
#[derive(Clone, Debug)]
pub enum N_0t_vbr {
    Var0(N_0t_vbrVar0),
}
impl N_0t_vbr {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_vbrVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:589:1"]
#[derive(Clone, Debug)]
pub struct U_2t_M0_dispr01Var0 {
    disp: i64,
    M_0: u8,
}
impl U_2t_M0_dispr01Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { disp, M_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Signed(true, *disp),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*M_0).unwrap()),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_2 = token_parser.U_2();
            let M_0 = token_parser.M_0();
            *context = context_current;
            Some(((), (U_2, M_0), block_len))
        };
        let ((), (U_2, M_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut disp = 0i64;
        disp = (i64::try_from(U_2).unwrap() * (1u64 as i64));
        let disp = 0i64;
        *context = context_current;
        Some((inst_len, Self { disp, M_0 }))
    }
}
#[doc = "Table U_2t_M0_dispr01"]
#[derive(Clone, Debug)]
pub enum U_2t_M0_dispr01 {
    Var0(U_2t_M0_dispr01Var0),
}
impl U_2t_M0_dispr01 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_2t_M0_dispr01Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:575:1"]
#[derive(Clone, Debug)]
pub struct DRM_1tVar0 {
    DRM_1: u8,
}
impl DRM_1tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { DRM_1 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590061630544(usize::try_from(*DRM_1).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let DRM_1 = token_parser.DRM_1();
            *context = context_current;
            Some(((), (DRM_1), block_len))
        };
        let ((), (DRM_1), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { DRM_1 }))
    }
}
#[doc = "Table DRM_1t"]
#[derive(Clone, Debug)]
pub enum DRM_1t {
    Var0(DRM_1tVar0),
}
impl DRM_1t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = DRM_1tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:543:1"]
#[derive(Clone, Debug)]
pub struct N_0t_machVar0 {
    N_0: u8,
}
impl N_0t_machVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::MACH),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_mach"]
#[derive(Clone, Debug)]
pub enum N_0t_mach {
    Var0(N_0t_machVar0),
}
impl N_0t_mach {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_machVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:467:1"]
#[derive(Clone, Debug)]
pub struct vbr_N_0tVar0 {
    N_0: u8,
}
impl vbr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::VBR),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table vbr_N_0t"]
#[derive(Clone, Debug)]
pub enum vbr_N_0t {
    Var0(vbr_N_0tVar0),
}
impl vbr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = vbr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:545:1"]
#[derive(Clone, Debug)]
pub struct N_0t_maclVar0 {
    N_0: u8,
}
impl N_0t_maclVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::MACL),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_macl"]
#[derive(Clone, Debug)]
pub enum N_0t_macl {
    Var0(N_0t_maclVar0),
}
impl N_0t_macl {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_maclVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:541:1"]
#[derive(Clone, Debug)]
pub struct fpscr_N_0tVar0 {
    N_0: u8,
}
impl fpscr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("FPSCR"),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table fpscr_N_0t"]
#[derive(Clone, Debug)]
pub enum fpscr_N_0t {
    Var0(fpscr_N_0tVar0),
}
impl fpscr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = fpscr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:457:1"]
#[derive(Clone, Debug)]
pub struct U_0t_r0Var0 {
    U_0: u8,
}
impl U_0t_r0Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Unsigned(true, u64::try_from(*U_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { U_0 }))
    }
}
#[doc = "Table U_0t_r0"]
#[derive(Clone, Debug)]
pub enum U_0t_r0 {
    Var0(U_0t_r0Var0),
}
impl U_0t_r0 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0t_r0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:539:1"]
#[derive(Clone, Debug)]
pub struct fpul_N_0tVar0 {
    N_0: u8,
}
impl fpul_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table fpul_N_0t"]
#[derive(Clone, Debug)]
pub enum fpul_N_0t {
    Var0(fpul_N_0tVar0),
}
impl fpul_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = fpul_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:555:1"]
#[derive(Clone, Debug)]
pub struct N_0t_macl1Var0 {
    N_0: u8,
}
impl N_0t_macl1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::MACL),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_macl1"]
#[derive(Clone, Debug)]
pub enum N_0t_macl1 {
    Var0(N_0t_macl1Var0),
}
impl N_0t_macl1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_macl1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:535:1"]
#[derive(Clone, Debug)]
pub struct macl_N_0tVar0 {
    N_0: u8,
}
impl macl_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::MACL),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table macl_N_0t"]
#[derive(Clone, Debug)]
pub enum macl_N_0t {
    Var0(macl_N_0tVar0),
}
impl macl_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = macl_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:611:1"]
#[derive(Clone, Debug)]
pub struct N_0txxVar0 {
    N_0: u8,
}
impl N_0txxVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(",@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0txx"]
#[derive(Clone, Debug)]
pub enum N_0txx {
    Var0(N_0txxVar0),
}
impl N_0txx {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0txxVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:596:1"]
#[derive(Clone, Debug)]
pub struct U_0t_gbr_at_1Var0 {
    disp: i64,
}
impl U_0t_gbr_at_1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { disp } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Signed(true, *disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut disp = 0i64;
        disp = (i64::try_from(U_0).unwrap() * (1u64 as i64));
        let disp = 0i64;
        *context = context_current;
        Some((inst_len, Self { disp }))
    }
}
#[doc = "Table U_0t_gbr_at_1"]
#[derive(Clone, Debug)]
pub enum U_0t_gbr_at_1 {
    Var0(U_0t_gbr_at_1Var0),
}
impl U_0t_gbr_at_1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0t_gbr_at_1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:510:1"]
#[derive(Clone, Debug)]
pub struct N_0t_vbr1Var0 {
    N_0: u8,
}
impl N_0t_vbr1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::VBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_vbr1"]
#[derive(Clone, Debug)]
pub enum N_0t_vbr1 {
    Var0(N_0t_vbr1Var0),
}
impl N_0t_vbr1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_vbr1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:567:1"]
#[derive(Clone, Debug)]
pub struct M_0t_atVar0 {
    M_0: u8,
}
impl M_0t_atVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*M_0).unwrap()),
            DisplayElement::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let M_0 = token_parser.M_0();
            *context = context_current;
            Some(((), (M_0), block_len))
        };
        let ((), (M_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { M_0 }))
    }
}
#[doc = "Table M_0t_at"]
#[derive(Clone, Debug)]
pub enum M_0t_at {
    Var0(M_0t_atVar0),
}
impl M_0t_at {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = M_0t_atVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:514:1"]
#[derive(Clone, Debug)]
pub struct N_0t_spc1Var0 {
    N_0: u8,
}
impl N_0t_spc1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::SPC),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_spc1"]
#[derive(Clone, Debug)]
pub enum N_0t_spc1 {
    Var0(N_0t_spc1Var0),
}
impl N_0t_spc1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_spc1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:537:1"]
#[derive(Clone, Debug)]
pub struct pr_N_0tVar0 {
    N_0: u8,
}
impl pr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::PR),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table pr_N_0t"]
#[derive(Clone, Debug)]
pub enum pr_N_0t {
    Var0(pr_N_0tVar0),
}
impl pr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = pr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:605:1"]
#[derive(Clone, Debug)]
pub struct U_0t_2pcVar0 {
    dest: i64,
}
impl U_0t_2pcVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { dest } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *dest)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut dest = 0i64;
        dest = (((i64::try_from(inst_start).unwrap()
            + i64::try_from(U_0).unwrap())
            * (2u64 as i64))
            + (4u64 as i64));
        let dest = 0i64;
        *context = context_current;
        Some((inst_len, Self { dest }))
    }
}
#[doc = "Table U_0t_2pc"]
#[derive(Clone, Debug)]
pub enum U_0t_2pc {
    Var0(U_0t_2pcVar0),
}
impl U_0t_2pc {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0t_2pcVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:525:1"]
#[derive(Clone, Debug)]
pub struct mach_tVar0 {}
impl mach_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::MACH)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table mach_t"]
#[derive(Clone, Debug)]
pub enum mach_t {
    Var0(mach_tVar0),
}
impl mach_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = mach_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:573:1"]
#[derive(Clone, Debug)]
pub struct FRN_0tVar0 {
    FRN_0: u8,
}
impl FRN_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590182964544(usize::try_from(*FRN_0).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FRN_0 = token_parser.FRN_0();
            *context = context_current;
            Some(((), (FRN_0), block_len))
        };
        let ((), (FRN_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { FRN_0 }))
    }
}
#[doc = "Table FRN_0t"]
#[derive(Clone, Debug)]
pub enum FRN_0t {
    Var0(FRN_0tVar0),
}
impl FRN_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = FRN_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:527:1"]
#[derive(Clone, Debug)]
pub struct macl_tVar0 {}
impl macl_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::MACL)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table macl_t"]
#[derive(Clone, Debug)]
pub enum macl_t {
    Var0(macl_tVar0),
}
impl macl_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = macl_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:581:1"]
#[derive(Clone, Debug)]
pub struct FVN_2tVar0 {
    FVN_2: u8,
}
impl FVN_2tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FVN_2 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590056407392(usize::try_from(*FVN_2).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FVN_2 = token_parser.FVN_2();
            *context = context_current;
            Some(((), (FVN_2), block_len))
        };
        let ((), (FVN_2), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { FVN_2 }))
    }
}
#[doc = "Table FVN_2t"]
#[derive(Clone, Debug)]
pub enum FVN_2t {
    Var0(FVN_2tVar0),
}
impl FVN_2t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = FVN_2tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:493:1"]
#[derive(Clone, Debug)]
pub struct N_0t_gbrVar0 {
    N_0: u8,
}
impl N_0t_gbrVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_gbr"]
#[derive(Clone, Debug)]
pub enum N_0t_gbr {
    Var0(N_0t_gbrVar0),
}
impl N_0t_gbr {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_gbrVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:590:1"]
#[derive(Clone, Debug)]
pub struct U_2t_M0_dispr02Var0 {
    disp: i64,
    M_0: u8,
}
impl U_2t_M0_dispr02Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { disp, M_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Signed(true, *disp),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*M_0).unwrap()),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_2 = token_parser.U_2();
            let M_0 = token_parser.M_0();
            *context = context_current;
            Some(((), (M_0, U_2), block_len))
        };
        let ((), (M_0, U_2), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut disp = 0i64;
        disp = (i64::try_from(U_2).unwrap() * (2u64 as i64));
        let disp = 0i64;
        *context = context_current;
        Some((inst_len, Self { disp, M_0 }))
    }
}
#[doc = "Table U_2t_M0_dispr02"]
#[derive(Clone, Debug)]
pub enum U_2t_M0_dispr02 {
    Var0(U_2t_M0_dispr02Var0),
}
impl U_2t_M0_dispr02 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_2t_M0_dispr02Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:563:1"]
#[derive(Clone, Debug)]
pub struct M_0t_at1Var0 {
    M_0: u8,
}
impl M_0t_at1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*M_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let M_0 = token_parser.M_0();
            *context = context_current;
            Some(((), (M_0), block_len))
        };
        let ((), (M_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { M_0 }))
    }
}
#[doc = "Table M_0t_at1"]
#[derive(Clone, Debug)]
pub enum M_0t_at1 {
    Var0(M_0t_at1Var0),
}
impl M_0t_at1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = M_0t_at1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:597:1"]
#[derive(Clone, Debug)]
pub struct U_0t_gbr_at_2Var0 {
    disp: i64,
}
impl U_0t_gbr_at_2Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { disp } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Signed(true, *disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut disp = 0i64;
        disp = (i64::try_from(U_0).unwrap() * (2u64 as i64));
        let disp = 0i64;
        *context = context_current;
        Some((inst_len, Self { disp }))
    }
}
#[doc = "Table U_0t_gbr_at_2"]
#[derive(Clone, Debug)]
pub enum U_0t_gbr_at_2 {
    Var0(U_0t_gbr_at_2Var0),
}
impl U_0t_gbr_at_2 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0t_gbr_at_2Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:508:1"]
#[derive(Clone, Debug)]
pub struct N_0t_gbr1Var0 {
    N_0: u8,
}
impl N_0t_gbr1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::GBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_gbr1"]
#[derive(Clone, Debug)]
pub enum N_0t_gbr1 {
    Var0(N_0t_gbr1Var0),
}
impl N_0t_gbr1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_gbr1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:452:1"]
#[derive(Clone, Debug)]
pub struct U_0tVar0 {
    U_0: u8,
}
impl U_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Unsigned(true, u64::try_from(*U_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { U_0 }))
    }
}
#[doc = "Table U_0t"]
#[derive(Clone, Debug)]
pub enum U_0t {
    Var0(U_0tVar0),
}
impl U_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:491:1"]
#[derive(Clone, Debug)]
pub struct N_0t_srVar0 {
    N_0: u8,
}
impl N_0t_srVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::SR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_sr"]
#[derive(Clone, Debug)]
pub enum N_0t_sr {
    Var0(N_0t_srVar0),
}
impl N_0t_sr {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_srVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:579:1"]
#[derive(Clone, Debug)]
pub struct FVM_2tVar0 {
    FVM_2: u8,
}
impl FVM_2tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FVM_2 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590056407392(usize::try_from(*FVM_2).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FVM_2 = token_parser.FVM_2();
            *context = context_current;
            Some(((), (FVM_2), block_len))
        };
        let ((), (FVM_2), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { FVM_2 }))
    }
}
#[doc = "Table FVM_2t"]
#[derive(Clone, Debug)]
pub enum FVM_2t {
    Var0(FVM_2tVar0),
}
impl FVM_2t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = FVM_2tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:519:1"]
#[derive(Clone, Debug)]
pub struct N_0t_bank1Var0 {
    N_0: u8,
}
impl N_0t_bank1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_bank1"]
#[derive(Clone, Debug)]
pub enum N_0t_bank1 {
    Var0(N_0t_bank1Var0),
}
impl N_0t_bank1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_bank1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:479:1"]
#[derive(Clone, Debug)]
pub struct gbr_tVar0 {}
impl gbr_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::GBR)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table gbr_t"]
#[derive(Clone, Debug)]
pub enum gbr_t {
    Var0(gbr_tVar0),
}
impl gbr_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = gbr_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:497:1"]
#[derive(Clone, Debug)]
pub struct N_0t_ssrVar0 {
    N_0: u8,
}
impl N_0t_ssrVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::SSR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_ssr"]
#[derive(Clone, Debug)]
pub enum N_0t_ssr {
    Var0(N_0t_ssrVar0),
}
impl N_0t_ssr {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_ssrVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:465:1"]
#[derive(Clone, Debug)]
pub struct gbr_N_0tVar0 {
    N_0: u8,
}
impl gbr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::GBR),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table gbr_N_0t"]
#[derive(Clone, Debug)]
pub enum gbr_N_0t {
    Var0(gbr_N_0tVar0),
}
impl gbr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = gbr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:553:1"]
#[derive(Clone, Debug)]
pub struct N_0t_mach1Var0 {
    N_0: u8,
}
impl N_0t_mach1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::MACH),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_mach1"]
#[derive(Clone, Debug)]
pub enum N_0t_mach1 {
    Var0(N_0t_mach1Var0),
}
impl N_0t_mach1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_mach1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:565:1"]
#[derive(Clone, Debug)]
pub struct N_0t_at1Var0 {
    N_0: u8,
}
impl N_0t_at1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_at1"]
#[derive(Clone, Debug)]
pub enum N_0t_at1 {
    Var0(N_0t_at1Var0),
}
impl N_0t_at1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_at1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:583:1"]
#[derive(Clone, Debug)]
pub struct N_0t_at_with_r0Var0 {
    N_0: u8,
}
impl N_0t_at_with_r0Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_at_with_r0"]
#[derive(Clone, Debug)]
pub enum N_0t_at_with_r0 {
    Var0(N_0t_at_with_r0Var0),
}
impl N_0t_at_with_r0 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_at_with_r0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:517:1"]
#[derive(Clone, Debug)]
pub struct N_0t_dbr1Var0 {
    N_0: u8,
}
impl N_0t_dbr1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::DBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_dbr1"]
#[derive(Clone, Debug)]
pub enum N_0t_dbr1 {
    Var0(N_0t_dbr1Var0),
}
impl N_0t_dbr1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_dbr1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:453:1"]
#[derive(Clone, Debug)]
pub struct U_0t1Var0 {
    U_0: u8,
}
impl U_0t1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0 } = self;
        let extend: [DisplayElement; 7usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Unsigned(true, u64::try_from(*U_0).unwrap()),
            DisplayElement::Literal(",@("),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { U_0 }))
    }
}
#[doc = "Table U_0t1"]
#[derive(Clone, Debug)]
pub enum U_0t1 {
    Var0(U_0t1Var0),
}
impl U_0t1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0t1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:571:1"]
#[derive(Clone, Debug)]
pub struct FRM_0tVar0 {
    FRM_0: u8,
}
impl FRM_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRM_0 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590182964544(usize::try_from(*FRM_0).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FRM_0 = token_parser.FRM_0();
            *context = context_current;
            Some(((), (FRM_0), block_len))
        };
        let ((), (FRM_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { FRM_0 }))
    }
}
#[doc = "Table FRM_0t"]
#[derive(Clone, Debug)]
pub enum FRM_0t {
    Var0(FRM_0tVar0),
}
impl FRM_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = FRM_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:489:1"]
#[derive(Clone, Debug)]
pub struct dbr_tVar0 {}
impl dbr_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::DBR)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table dbr_t"]
#[derive(Clone, Debug)]
pub enum dbr_t {
    Var0(dbr_tVar0),
}
impl dbr_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = dbr_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:607:1"]
#[derive(Clone, Debug)]
pub struct U_0t_4pcVar0 {
    dest: i64,
}
impl U_0t_4pcVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { dest } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *dest)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut dest = 0i64;
        dest = ((((i64::try_from(inst_start).unwrap()
            & (4294967292u64 as i64))
            + i64::try_from(U_0).unwrap())
            * (4u64 as i64))
            + (4u64 as i64));
        let dest = 0i64;
        *context = context_current;
        Some((inst_len, Self { dest }))
    }
}
#[doc = "Table U_0t_4pc"]
#[derive(Clone, Debug)]
pub enum U_0t_4pc {
    Var0(U_0t_4pcVar0),
}
impl U_0t_4pc {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0t_4pcVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:609:1"]
#[derive(Clone, Debug)]
pub struct BANKtVar0 {
    BANK: u8,
}
impl BANKtVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { BANK } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590034762480(usize::try_from(*BANK).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let BANK = token_parser.BANK();
            *context = context_current;
            Some(((), (BANK), block_len))
        };
        let ((), (BANK), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { BANK }))
    }
}
#[doc = "Table BANKt"]
#[derive(Clone, Debug)]
pub enum BANKt {
    Var0(BANKtVar0),
}
impl BANKt {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = BANKtVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:475:1"]
#[derive(Clone, Debug)]
pub struct dbr_N_0tVar0 {
    N_0: u8,
}
impl dbr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::DBR),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table dbr_N_0t"]
#[derive(Clone, Debug)]
pub enum dbr_N_0t {
    Var0(dbr_N_0tVar0),
}
impl dbr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = dbr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:455:1"]
#[derive(Clone, Debug)]
pub struct I_0t_r0Var0 {
    I_0: i8,
}
impl I_0t_r0Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Signed(true, i64::try_from(*I_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let I_0 = token_parser.I_0();
            *context = context_current;
            Some(((), (I_0), block_len))
        };
        let ((), (I_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { I_0 }))
    }
}
#[doc = "Table I_0t_r0"]
#[derive(Clone, Debug)]
pub enum I_0t_r0 {
    Var0(I_0t_r0Var0),
}
impl I_0t_r0 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = I_0t_r0Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:481:1"]
#[derive(Clone, Debug)]
pub struct vbr_tVar0 {}
impl vbr_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::VBR)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table vbr_t"]
#[derive(Clone, Debug)]
pub enum vbr_t {
    Var0(vbr_tVar0),
}
impl vbr_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = vbr_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:598:1"]
#[derive(Clone, Debug)]
pub struct U_0t_gbr_at_4Var0 {
    disp: i64,
}
impl U_0t_gbr_at_4Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { disp } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Signed(true, *disp),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::GBR),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_0 = token_parser.U_0();
            *context = context_current;
            Some(((), (U_0), block_len))
        };
        let ((), (U_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut disp = 0i64;
        disp = (i64::try_from(U_0).unwrap() * (4u64 as i64));
        let disp = 0i64;
        *context = context_current;
        Some((inst_len, Self { disp }))
    }
}
#[doc = "Table U_0t_gbr_at_4"]
#[derive(Clone, Debug)]
pub enum U_0t_gbr_at_4 {
    Var0(U_0t_gbr_at_4Var0),
}
impl U_0t_gbr_at_4 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_0t_gbr_at_4Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:463:1"]
#[derive(Clone, Debug)]
pub struct sr_N_0tVar0 {
    N_0: u8,
}
impl sr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::SR),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table sr_N_0t"]
#[derive(Clone, Debug)]
pub enum sr_N_0t {
    Var0(sr_N_0tVar0),
}
impl sr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = sr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:512:1"]
#[derive(Clone, Debug)]
pub struct N_0t_ssr1Var0 {
    N_0: u8,
}
impl N_0t_ssr1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::SSR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_ssr1"]
#[derive(Clone, Debug)]
pub enum N_0t_ssr1 {
    Var0(N_0t_ssr1Var0),
}
impl N_0t_ssr1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_ssr1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:593:1"]
#[derive(Clone, Debug)]
pub struct U_2t_N0_dispr04Var0 {
    disp: i64,
    N_0: u8,
}
impl U_2t_N0_dispr04Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { disp, N_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Signed(true, *disp),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_2 = token_parser.U_2();
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0, U_2), block_len))
        };
        let ((), (N_0, U_2), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut disp = 0i64;
        disp = (i64::try_from(U_2).unwrap() * (4u64 as i64));
        let disp = 0i64;
        *context = context_current;
        Some((inst_len, Self { disp, N_0 }))
    }
}
#[doc = "Table U_2t_N0_dispr04"]
#[derive(Clone, Debug)]
pub enum U_2t_N0_dispr04 {
    Var0(U_2t_N0_dispr04Var0),
}
impl U_2t_N0_dispr04 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_2t_N0_dispr04Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:551:1"]
#[derive(Clone, Debug)]
pub struct N_0t_fpscrVar0 {
    N_0: u8,
}
impl N_0t_fpscrVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(",fpscr"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_fpscr"]
#[derive(Clone, Debug)]
pub enum N_0t_fpscr {
    Var0(N_0t_fpscrVar0),
}
impl N_0t_fpscr {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_fpscrVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1533:1"]
#[derive(Clone, Debug)]
pub struct instructionVar0 {
    XMTRX_t: XMTRX_t,
    FVN_2t: FVN_2t,
}
impl instructionVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { XMTRX_t, FVN_2t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ftrv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        XMTRX_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FVN_2t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FVN_2t = if let Some((len, table)) = FVN_2t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_8() != (509u64 as i64) as u16 {
                return None;
            }
            let XMTRX_t = if let Some((len, table)) = XMTRX_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((FVN_2t, XMTRX_t), (), block_len))
        };
        let ((mut FVN_2t, mut XMTRX_t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        XMTRX_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FVN_2t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { XMTRX_t, FVN_2t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1189:1"]
#[derive(Clone, Debug)]
pub struct instructionVar1 {
    DRN_1t: DRN_1t,
}
impl instructionVar1 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { DRN_1t } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("fcnvsd"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        DRN_1t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_5() != (173u64 as i64) as u16 {
                return None;
            }
            *context = context_current;
            Some(((DRN_1t), (), block_len))
        };
        let ((mut DRN_1t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        DRN_1t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { DRN_1t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1176:1"]
#[derive(Clone, Debug)]
pub struct instructionVar2 {
    DRN_1t: DRN_1t,
}
impl instructionVar2 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { DRN_1t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fcnvds"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        DRN_1t.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_5() != (189u64 as i64) as u16 {
                return None;
            }
            *context = context_current;
            Some(((DRN_1t), (), block_len))
        };
        let ((mut DRN_1t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        DRN_1t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { DRN_1t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2976:1"]
#[derive(Clone, Debug)]
pub struct instructionVar3 {
    N_0t_at1: N_0t_at1,
}
impl instructionVar3 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("tas.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (27u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at1), (), block_len))
        };
        let ((mut N_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2899:1"]
#[derive(Clone, Debug)]
pub struct instructionVar4 {
    fpscr_t: fpscr_t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar4 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            fpscr_t,
            N_0t_at_neg,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        fpscr_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (98u64 as i64) as u8 {
                return None;
            }
            let fpscr_t = if let Some((len, table)) = fpscr_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((fpscr_t, N_0t_at_neg), (), block_len))
        };
        let ((mut fpscr_t, mut N_0t_at_neg), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        fpscr_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                fpscr_t,
                N_0t_at_neg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2888:1"]
#[derive(Clone, Debug)]
pub struct instructionVar5 {
    N_0t_at_neg: N_0t_at_neg,
    fpul_t: fpul_t,
}
impl instructionVar5 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            N_0t_at_neg,
            fpul_t,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        fpul_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (82u64 as i64) as u8 {
                return None;
            }
            let fpul_t = if let Some((len, table)) = fpul_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((fpul_t, N_0t_at_neg), (), block_len))
        };
        let ((mut fpul_t, mut N_0t_at_neg), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        fpul_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                N_0t_at_neg,
                fpul_t,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2877:1"]
#[derive(Clone, Debug)]
pub struct instructionVar6 {
    fpscr_N_0t: fpscr_N_0t,
}
impl instructionVar6 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { fpscr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        fpscr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let fpscr_N_0t = if let Some((len, table)) = fpscr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (106u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((fpscr_N_0t), (), block_len))
        };
        let ((mut fpscr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        fpscr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { fpscr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2867:1"]
#[derive(Clone, Debug)]
pub struct instructionVar7 {
    fpul_N_0t: fpul_N_0t,
}
impl instructionVar7 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { fpul_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        fpul_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let fpul_N_0t = if let Some((len, table)) = fpul_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (90u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((fpul_N_0t), (), block_len))
        };
        let ((mut fpul_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        fpul_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { fpul_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2856:1"]
#[derive(Clone, Debug)]
pub struct instructionVar8 {
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar8 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at_neg } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::PR),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (34u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_neg), (), block_len))
        };
        let ((mut N_0t_at_neg), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at_neg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2845:1"]
#[derive(Clone, Debug)]
pub struct instructionVar9 {
    macl_t: macl_t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar9 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            macl_t,
            N_0t_at_neg,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        macl_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (18u64 as i64) as u8 {
                return None;
            }
            let macl_t = if let Some((len, table)) = macl_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((macl_t, N_0t_at_neg), (), block_len))
        };
        let ((mut macl_t, mut N_0t_at_neg), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        macl_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                macl_t,
                N_0t_at_neg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2834:1"]
#[derive(Clone, Debug)]
pub struct instructionVar10 {
    mach_t: mach_t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar10 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            mach_t,
            N_0t_at_neg,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("sts.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        mach_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (2u64 as i64) as u8 {
                return None;
            }
            let mach_t = if let Some((len, table)) = mach_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t_at_neg, mach_t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut mach_t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        mach_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                mach_t,
                N_0t_at_neg,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2824:1"]
#[derive(Clone, Debug)]
pub struct instructionVar11 {
    pr_N_0t: pr_N_0t,
}
impl instructionVar11 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { pr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        pr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let pr_N_0t = if let Some((len, table)) = pr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (42u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((pr_N_0t), (), block_len))
        };
        let ((mut pr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        pr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { pr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2814:1"]
#[derive(Clone, Debug)]
pub struct instructionVar12 {
    macl_N_0t: macl_N_0t,
}
impl instructionVar12 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { macl_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        macl_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let macl_N_0t = if let Some((len, table)) = macl_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (26u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((macl_N_0t), (), block_len))
        };
        let ((mut macl_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        macl_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { macl_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2804:1"]
#[derive(Clone, Debug)]
pub struct instructionVar13 {
    mach_N_0t: mach_N_0t,
}
impl instructionVar13 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { mach_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sts"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        mach_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let mach_N_0t = if let Some((len, table)) = mach_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (10u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((mach_N_0t), (), block_len))
        };
        let ((mut mach_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        mach_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { mach_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2783:1"]
#[derive(Clone, Debug)]
pub struct instructionVar14 {
    dbr_t: dbr_t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar14 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { dbr_t, N_0t_at_neg } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        dbr_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (242u64 as i64) as u8 {
                return None;
            }
            let dbr_t = if let Some((len, table)) = dbr_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t_at_neg, dbr_t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut dbr_t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        dbr_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { dbr_t, N_0t_at_neg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2773:1"]
#[derive(Clone, Debug)]
pub struct instructionVar15 {
    N_0t_at_neg: N_0t_at_neg,
    sgr_t: sgr_t,
}
impl instructionVar15 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at_neg, sgr_t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        sgr_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (50u64 as i64) as u8 {
                return None;
            }
            let sgr_t = if let Some((len, table)) = sgr_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t_at_neg, sgr_t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut sgr_t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        sgr_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at_neg, sgr_t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2763:1"]
#[derive(Clone, Debug)]
pub struct instructionVar16 {
    N_0t_at_neg: N_0t_at_neg,
    spc_t: spc_t,
}
impl instructionVar16 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at_neg, spc_t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        spc_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (67u64 as i64) as u8 {
                return None;
            }
            let spc_t = if let Some((len, table)) = spc_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t_at_neg, spc_t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut spc_t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        spc_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at_neg, spc_t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2753:1"]
#[derive(Clone, Debug)]
pub struct instructionVar17 {
    N_0t_at_neg: N_0t_at_neg,
    ssr_t: ssr_t,
}
impl instructionVar17 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at_neg, ssr_t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        ssr_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (51u64 as i64) as u8 {
                return None;
            }
            let ssr_t = if let Some((len, table)) = ssr_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((ssr_t, N_0t_at_neg), (), block_len))
        };
        let ((mut ssr_t, mut N_0t_at_neg), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        ssr_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at_neg, ssr_t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2743:1"]
#[derive(Clone, Debug)]
pub struct instructionVar18 {
    N_0t_at_neg: N_0t_at_neg,
    vbr_t: vbr_t,
}
impl instructionVar18 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at_neg, vbr_t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        vbr_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (35u64 as i64) as u8 {
                return None;
            }
            let vbr_t = if let Some((len, table)) = vbr_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t_at_neg, vbr_t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut vbr_t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        vbr_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at_neg, vbr_t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2732:1"]
#[derive(Clone, Debug)]
pub struct instructionVar19 {
    gbr_t: gbr_t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar19 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { gbr_t, N_0t_at_neg } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        gbr_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (19u64 as i64) as u8 {
                return None;
            }
            let gbr_t = if let Some((len, table)) = gbr_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t_at_neg, gbr_t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut gbr_t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        gbr_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { gbr_t, N_0t_at_neg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2720:1"]
#[derive(Clone, Debug)]
pub struct instructionVar20 {
    sr_t: sr_t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar20 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { sr_t, N_0t_at_neg } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        sr_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (3u64 as i64) as u8 {
                return None;
            }
            let sr_t = if let Some((len, table)) = sr_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((sr_t, N_0t_at_neg), (), block_len))
        };
        let ((mut sr_t, mut N_0t_at_neg), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        sr_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { sr_t, N_0t_at_neg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2701:1"]
#[derive(Clone, Debug)]
pub struct instructionVar21 {
    dbr_N_0t: dbr_N_0t,
}
impl instructionVar21 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { dbr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        dbr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let dbr_N_0t = if let Some((len, table)) = dbr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (250u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((dbr_N_0t), (), block_len))
        };
        let ((mut dbr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        dbr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { dbr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2692:1"]
#[derive(Clone, Debug)]
pub struct instructionVar22 {
    sgr_N_0t: sgr_N_0t,
}
impl instructionVar22 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { sgr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        sgr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let sgr_N_0t = if let Some((len, table)) = sgr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (58u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((sgr_N_0t), (), block_len))
        };
        let ((mut sgr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        sgr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { sgr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2683:1"]
#[derive(Clone, Debug)]
pub struct instructionVar23 {
    spc_N_0t: spc_N_0t,
}
impl instructionVar23 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { spc_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        spc_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let spc_N_0t = if let Some((len, table)) = spc_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (66u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((spc_N_0t), (), block_len))
        };
        let ((mut spc_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        spc_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { spc_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2673:1"]
#[derive(Clone, Debug)]
pub struct instructionVar24 {
    ssr_N_0t: ssr_N_0t,
}
impl instructionVar24 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { ssr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        ssr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let ssr_N_0t = if let Some((len, table)) = ssr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (50u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((ssr_N_0t), (), block_len))
        };
        let ((mut ssr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        ssr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { ssr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2664:1"]
#[derive(Clone, Debug)]
pub struct instructionVar25 {
    vbr_N_0t: vbr_N_0t,
}
impl instructionVar25 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { vbr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        vbr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let vbr_N_0t = if let Some((len, table)) = vbr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (34u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((vbr_N_0t), (), block_len))
        };
        let ((mut vbr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        vbr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { vbr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2654:1"]
#[derive(Clone, Debug)]
pub struct instructionVar26 {
    gbr_N_0t: gbr_N_0t,
}
impl instructionVar26 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { gbr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        gbr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let gbr_N_0t = if let Some((len, table)) = gbr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (18u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((gbr_N_0t), (), block_len))
        };
        let ((mut gbr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        gbr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { gbr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2643:1"]
#[derive(Clone, Debug)]
pub struct instructionVar27 {
    sr_N_0t: sr_N_0t,
}
impl instructionVar27 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { sr_N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        sr_N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let sr_N_0t = if let Some((len, table)) = sr_N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((sr_N_0t), (), block_len))
        };
        let ((mut sr_N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        sr_N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { sr_N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2626:1"]
#[derive(Clone, Debug)]
pub struct instructionVar28 {
    N_0t: N_0t,
}
impl instructionVar28 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shlr16"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (41u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2616:1"]
#[derive(Clone, Debug)]
pub struct instructionVar29 {
    N_0t: N_0t,
}
impl instructionVar29 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shlr8"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (25u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2606:1"]
#[derive(Clone, Debug)]
pub struct instructionVar30 {
    N_0t: N_0t,
}
impl instructionVar30 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shlr2"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (9u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2595:1"]
#[derive(Clone, Debug)]
pub struct instructionVar31 {
    N_0t: N_0t,
}
impl instructionVar31 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shlr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2585:1"]
#[derive(Clone, Debug)]
pub struct instructionVar32 {
    N_0t: N_0t,
}
impl instructionVar32 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shll16"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (40u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2576:1"]
#[derive(Clone, Debug)]
pub struct instructionVar33 {
    N_0t: N_0t,
}
impl instructionVar33 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shll8"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (24u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2566:1"]
#[derive(Clone, Debug)]
pub struct instructionVar34 {
    N_0t: N_0t,
}
impl instructionVar34 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shll2"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (8u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2555:1"]
#[derive(Clone, Debug)]
pub struct instructionVar35 {
    N_0t: N_0t,
}
impl instructionVar35 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shll"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2529:1"]
#[derive(Clone, Debug)]
pub struct instructionVar36 {
    N_0t: N_0t,
}
impl instructionVar36 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shar"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (33u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2518:1"]
#[derive(Clone, Debug)]
pub struct instructionVar37 {
    N_0t: N_0t,
}
impl instructionVar37 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shal"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (32u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2447:1"]
#[derive(Clone, Debug)]
pub struct instructionVar38 {
    N_0t: N_0t,
}
impl instructionVar38 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("rotr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (5u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2436:1"]
#[derive(Clone, Debug)]
pub struct instructionVar39 {
    N_0t: N_0t,
}
impl instructionVar39 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("rotl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2424:1"]
#[derive(Clone, Debug)]
pub struct instructionVar40 {
    N_0t: N_0t,
}
impl instructionVar40 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("rotcr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (37u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2413:1"]
#[derive(Clone, Debug)]
pub struct instructionVar41 {
    N_0t: N_0t,
}
impl instructionVar41 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("rotcl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (36u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2406:1"]
#[derive(Clone, Debug)]
pub struct instructionVar42 {
    N_0tjmp: N_0tjmp,
}
impl instructionVar42 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0tjmp } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("pref"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0tjmp.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0tjmp = if let Some((len, table)) = N_0tjmp::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (131u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0tjmp), (), block_len))
        };
        let ((mut N_0tjmp), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0tjmp.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0tjmp }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2369:1"]
#[derive(Clone, Debug)]
pub struct instructionVar43 {
    N_0t_at1: N_0t_at1,
}
impl instructionVar43 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ocbwb"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (179u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at1), (), block_len))
        };
        let ((mut N_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2360:1"]
#[derive(Clone, Debug)]
pub struct instructionVar44 {
    N_0t_at1: N_0t_at1,
}
impl instructionVar44 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ocbp"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (163u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at1), (), block_len))
        };
        let ((mut N_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2352:1"]
#[derive(Clone, Debug)]
pub struct instructionVar45 {
    N_0t_at1: N_0t_at1,
}
impl instructionVar45 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ocbi"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (147u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at1), (), block_len))
        };
        let ((mut N_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2271:1"]
#[derive(Clone, Debug)]
pub struct instructionVar46 {
    N_0t: N_0t,
}
impl instructionVar46 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("movt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (41u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2262:1"]
#[derive(Clone, Debug)]
pub struct instructionVar47 {
    N_0txx: N_0txx,
}
impl instructionVar47 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0txx } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("movca.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0txx.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0txx = if let Some((len, table)) = N_0txx::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (195u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0txx), (), block_len))
        };
        let ((mut N_0txx), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0txx.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0txx }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1844:1"]
#[derive(Clone, Debug)]
pub struct instructionVar48 {
    N_0t_pr1: N_0t_pr1,
}
impl instructionVar48 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_pr1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_pr1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_pr1 = if let Some((len, table)) = N_0t_pr1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (38u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_pr1), (), block_len))
        };
        let ((mut N_0t_pr1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_pr1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_pr1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1833:1"]
#[derive(Clone, Debug)]
pub struct instructionVar49 {
    N_0t_macl1: N_0t_macl1,
}
impl instructionVar49 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_macl1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_macl1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_macl1 = if let Some((len, table)) = N_0t_macl1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (22u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_macl1), (), block_len))
        };
        let ((mut N_0t_macl1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_macl1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_macl1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1822:1"]
#[derive(Clone, Debug)]
pub struct instructionVar50 {
    N_0t_mach1: N_0t_mach1,
}
impl instructionVar50 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_mach1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_mach1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_mach1 = if let Some((len, table)) = N_0t_mach1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_mach1), (), block_len))
        };
        let ((mut N_0t_mach1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_mach1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_mach1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1812:1"]
#[derive(Clone, Debug)]
pub struct instructionVar51 {
    N_0t_pr: N_0t_pr,
}
impl instructionVar51 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_pr } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_pr.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_pr = if let Some((len, table)) = N_0t_pr::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (42u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_pr), (), block_len))
        };
        let ((mut N_0t_pr), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_pr.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_pr }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1802:1"]
#[derive(Clone, Debug)]
pub struct instructionVar52 {
    N_0t_macl: N_0t_macl,
}
impl instructionVar52 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_macl } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_macl.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_macl = if let Some((len, table)) = N_0t_macl::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (26u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_macl), (), block_len))
        };
        let ((mut N_0t_macl), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_macl.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_macl }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1792:1"]
#[derive(Clone, Debug)]
pub struct instructionVar53 {
    N_0t_mach: N_0t_mach,
}
impl instructionVar53 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_mach } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_mach.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_mach = if let Some((len, table)) = N_0t_mach::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (10u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_mach), (), block_len))
        };
        let ((mut N_0t_mach), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_mach.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_mach }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1780:1"]
#[derive(Clone, Debug)]
pub struct instructionVar54 {
    N_0t_fpscr1: N_0t_fpscr1,
}
impl instructionVar54 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_fpscr1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_fpscr1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_fpscr1 = if let Some((len, table)) = N_0t_fpscr1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (102u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_fpscr1), (), block_len))
        };
        let ((mut N_0t_fpscr1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_fpscr1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_fpscr1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1768:1"]
#[derive(Clone, Debug)]
pub struct instructionVar55 {
    N_0t_fpscr: N_0t_fpscr,
}
impl instructionVar55 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_fpscr } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_fpscr.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_fpscr = if let Some((len, table)) = N_0t_fpscr::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (106u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_fpscr), (), block_len))
        };
        let ((mut N_0t_fpscr), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_fpscr.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_fpscr }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1756:1"]
#[derive(Clone, Debug)]
pub struct instructionVar56 {
    N_0t_fpul1: N_0t_fpul1,
}
impl instructionVar56 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_fpul1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("lds.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_fpul1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_fpul1 = if let Some((len, table)) = N_0t_fpul1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (86u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_fpul1), (), block_len))
        };
        let ((mut N_0t_fpul1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_fpul1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_fpul1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1746:1"]
#[derive(Clone, Debug)]
pub struct instructionVar57 {
    N_0t_fpul: N_0t_fpul,
}
impl instructionVar57 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_fpul } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("lds"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_fpul.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_fpul = if let Some((len, table)) = N_0t_fpul::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (90u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_fpul), (), block_len))
        };
        let ((mut N_0t_fpul), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_fpul.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_fpul }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1725:1"]
#[derive(Clone, Debug)]
pub struct instructionVar58 {
    N_0t_dbr1: N_0t_dbr1,
}
impl instructionVar58 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_dbr1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_dbr1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_dbr1 = if let Some((len, table)) = N_0t_dbr1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (246u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_dbr1), (), block_len))
        };
        let ((mut N_0t_dbr1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_dbr1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_dbr1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1715:1"]
#[derive(Clone, Debug)]
pub struct instructionVar59 {
    N_0t_spc1: N_0t_spc1,
}
impl instructionVar59 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_spc1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_spc1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_spc1 = if let Some((len, table)) = N_0t_spc1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (71u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_spc1), (), block_len))
        };
        let ((mut N_0t_spc1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_spc1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_spc1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1705:1"]
#[derive(Clone, Debug)]
pub struct instructionVar60 {
    N_0t_ssr1: N_0t_ssr1,
}
impl instructionVar60 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_ssr1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_ssr1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_ssr1 = if let Some((len, table)) = N_0t_ssr1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (55u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_ssr1), (), block_len))
        };
        let ((mut N_0t_ssr1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_ssr1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_ssr1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1695:1"]
#[derive(Clone, Debug)]
pub struct instructionVar61 {
    N_0t_vbr1: N_0t_vbr1,
}
impl instructionVar61 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_vbr1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_vbr1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_vbr1 = if let Some((len, table)) = N_0t_vbr1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (39u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_vbr1), (), block_len))
        };
        let ((mut N_0t_vbr1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_vbr1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_vbr1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1684:1"]
#[derive(Clone, Debug)]
pub struct instructionVar62 {
    N_0t_gbr1: N_0t_gbr1,
}
impl instructionVar62 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_gbr1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_gbr1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_gbr1 = if let Some((len, table)) = N_0t_gbr1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (23u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_gbr1), (), block_len))
        };
        let ((mut N_0t_gbr1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_gbr1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_gbr1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1672:1"]
#[derive(Clone, Debug)]
pub struct instructionVar63 {
    N_0t_sr1: N_0t_sr1,
}
impl instructionVar63 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_sr1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_sr1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_sr1 = if let Some((len, table)) = N_0t_sr1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_sr1), (), block_len))
        };
        let ((mut N_0t_sr1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_sr1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_sr1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1654:1"]
#[derive(Clone, Debug)]
pub struct instructionVar64 {
    N_0t_dbr: N_0t_dbr,
}
impl instructionVar64 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_dbr } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_dbr.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_dbr = if let Some((len, table)) = N_0t_dbr::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (250u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_dbr), (), block_len))
        };
        let ((mut N_0t_dbr), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_dbr.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_dbr }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1645:1"]
#[derive(Clone, Debug)]
pub struct instructionVar65 {
    N_0t_spc: N_0t_spc,
}
impl instructionVar65 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_spc } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_spc.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_spc = if let Some((len, table)) = N_0t_spc::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (78u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_spc), (), block_len))
        };
        let ((mut N_0t_spc), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_spc.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_spc }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1636:1"]
#[derive(Clone, Debug)]
pub struct instructionVar66 {
    N_0t_ssr: N_0t_ssr,
}
impl instructionVar66 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_ssr } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_ssr.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_ssr = if let Some((len, table)) = N_0t_ssr::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (62u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_ssr), (), block_len))
        };
        let ((mut N_0t_ssr), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_ssr.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_ssr }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1627:1"]
#[derive(Clone, Debug)]
pub struct instructionVar67 {
    N_0t_vbr: N_0t_vbr,
}
impl instructionVar67 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_vbr } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_vbr.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_vbr = if let Some((len, table)) = N_0t_vbr::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (46u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_vbr), (), block_len))
        };
        let ((mut N_0t_vbr), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_vbr.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_vbr }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1617:1"]
#[derive(Clone, Debug)]
pub struct instructionVar68 {
    N_0t_gbr: N_0t_gbr,
}
impl instructionVar68 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_gbr } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_gbr.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_gbr = if let Some((len, table)) = N_0t_gbr::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (30u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_gbr), (), block_len))
        };
        let ((mut N_0t_gbr), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_gbr.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_gbr }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1606:1"]
#[derive(Clone, Debug)]
pub struct instructionVar69 {
    N_0t_sr: N_0t_sr,
}
impl instructionVar69 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_sr } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_sr.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_sr = if let Some((len, table)) = N_0t_sr::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (14u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_sr), (), block_len))
        };
        let ((mut N_0t_sr), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_sr.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_sr }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1592:1"]
#[derive(Clone, Debug)]
pub struct instructionVar70 {
    N_0tjmp: N_0tjmp,
}
impl instructionVar70 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0tjmp } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("jsr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0tjmp.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0tjmp = if let Some((len, table)) = N_0tjmp::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (11u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0tjmp), (), block_len))
        };
        let ((mut N_0tjmp), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0tjmp.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0tjmp }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1579:1"]
#[derive(Clone, Debug)]
pub struct instructionVar71 {
    N_0tjmp: N_0tjmp,
}
impl instructionVar71 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0tjmp } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("jmp"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0tjmp.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0tjmp = if let Some((len, table)) = N_0tjmp::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (43u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0tjmp), (), block_len))
        };
        let ((mut N_0tjmp), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0tjmp.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0tjmp }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1570:1"]
#[derive(Clone, Debug)]
pub struct instructionVar72 {
    N_0t_at1: N_0t_at1,
}
impl instructionVar72 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("icbi"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (227u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at1), (), block_len))
        };
        let ((mut N_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1518:1"]
#[derive(Clone, Debug)]
pub struct instructionVar73 {
    FRN_0t: FRN_0t,
}
impl instructionVar73 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ftrc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (61u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRN_0t, DRN_1t), (), block_len))
        };
        let ((mut FRN_0t, mut DRN_1t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1493:1"]
#[derive(Clone, Debug)]
pub struct instructionVar74 {
    FRN_0t: FRN_0t,
}
impl instructionVar74 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("fsts"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (13u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRN_0t), (), block_len))
        };
        let ((mut FRN_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1478:1"]
#[derive(Clone, Debug)]
pub struct instructionVar75 {
    FRN_0t: FRN_0t,
}
impl instructionVar75 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fsqrt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (109u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRN_0t, DRN_1t), (), block_len))
        };
        let ((mut FRN_0t, mut DRN_1t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1262:1"]
#[derive(Clone, Debug)]
pub struct instructionVar76 {
    FRN_0t: FRN_0t,
}
impl instructionVar76 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("flds"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (29u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRN_0t), (), block_len))
        };
        let ((mut FRN_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1252:1"]
#[derive(Clone, Debug)]
pub struct instructionVar77 {
    FRN_0t: FRN_0t,
}
impl instructionVar77 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fldi1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (157u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRN_0t, DRN_1t), (), block_len))
        };
        let ((mut FRN_0t, mut DRN_1t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1239:1"]
#[derive(Clone, Debug)]
pub struct instructionVar78 {
    FRN_0t: FRN_0t,
}
impl instructionVar78 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fldi0"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (141u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((DRN_1t, FRN_0t), (), block_len))
        };
        let ((mut DRN_1t, mut FRN_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1217:1"]
#[derive(Clone, Debug)]
pub struct instructionVar79 {
    FVM_2t: FVM_2t,
    FVN_2t: FVN_2t,
}
impl instructionVar79 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FVM_2t, FVN_2t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fipr"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FVM_2t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FVN_2t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FVN_2t = if let Some((len, table)) = FVN_2t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FVM_2t = if let Some((len, table)) = FVM_2t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (237u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FVM_2t, FVN_2t), (), block_len))
        };
        let ((mut FVM_2t, mut FVN_2t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FVM_2t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FVN_2t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FVM_2t, FVN_2t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1064:1"]
#[derive(Clone, Debug)]
pub struct instructionVar80 {
    N_0t: N_0t,
}
impl instructionVar80 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("dt"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (16u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:869:1"]
#[derive(Clone, Debug)]
pub struct instructionVar81 {
    N_0t: N_0t,
}
impl instructionVar81 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/pz"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (17u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:860:1"]
#[derive(Clone, Debug)]
pub struct instructionVar82 {
    N_0t: N_0t,
}
impl instructionVar82 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/pl"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (21u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t), (), block_len))
        };
        let ((mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:3056:1"]
#[derive(Clone, Debug)]
pub struct instructionVar83 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar83 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("xtrct"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (13u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:3026:1"]
#[derive(Clone, Debug)]
pub struct instructionVar84 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar84 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("xor"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (10u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2997:1"]
#[derive(Clone, Debug)]
pub struct instructionVar85 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar85 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("tst"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (8u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2956:1"]
#[derive(Clone, Debug)]
pub struct instructionVar86 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar86 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("swap.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (9u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2944:1"]
#[derive(Clone, Debug)]
pub struct instructionVar87 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar87 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("swap.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (8u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2934:1"]
#[derive(Clone, Debug)]
pub struct instructionVar88 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar88 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("subv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (11u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2921:1"]
#[derive(Clone, Debug)]
pub struct instructionVar89 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar89 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("subc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (10u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2911:1"]
#[derive(Clone, Debug)]
pub struct instructionVar90 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar90 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("sub"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (8u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2538:1"]
#[derive(Clone, Debug)]
pub struct instructionVar91 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar91 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shld"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (13u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2502:1"]
#[derive(Clone, Debug)]
pub struct instructionVar92 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar92 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("shad"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (12u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2376:1"]
#[derive(Clone, Debug)]
pub struct instructionVar93 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar93 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("or"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (11u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2341:1"]
#[derive(Clone, Debug)]
pub struct instructionVar94 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar94 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("not"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2321:1"]
#[derive(Clone, Debug)]
pub struct instructionVar95 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar95 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("negc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (10u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2311:1"]
#[derive(Clone, Debug)]
pub struct instructionVar96 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar96 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("neg"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (11u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2301:1"]
#[derive(Clone, Debug)]
pub struct instructionVar97 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar97 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mulu.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (14u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2291:1"]
#[derive(Clone, Debug)]
pub struct instructionVar98 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar98 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("muls.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (15u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2281:1"]
#[derive(Clone, Debug)]
pub struct instructionVar99 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar99 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mul.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2092:1"]
#[derive(Clone, Debug)]
pub struct instructionVar100 {
    M_0t_at_with_r0: M_0t_at_with_r0,
    N_0t: N_0t,
}
impl instructionVar100 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            M_0t_at_with_r0,
            N_0t,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at_with_r0.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at_with_r0 = if let Some((len, table)) =
                M_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (14u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t_at_with_r0), (), block_len))
        };
        let ((mut N_0t, mut M_0t_at_with_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                M_0t_at_with_r0,
                N_0t,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2082:1"]
#[derive(Clone, Debug)]
pub struct instructionVar101 {
    M_0t_at_with_r0: M_0t_at_with_r0,
    N_0t: N_0t,
}
impl instructionVar101 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            M_0t_at_with_r0,
            N_0t,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at_with_r0.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at_with_r0 = if let Some((len, table)) =
                M_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (13u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at_with_r0, N_0t), (), block_len))
        };
        let ((mut M_0t_at_with_r0, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                M_0t_at_with_r0,
                N_0t,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2072:1"]
#[derive(Clone, Debug)]
pub struct instructionVar102 {
    N_0t: N_0t,
    M_0t_at_with_r0: M_0t_at_with_r0,
}
impl instructionVar102 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            N_0t,
            M_0t_at_with_r0,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at_with_r0.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at_with_r0 = if let Some((len, table)) =
                M_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (12u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t_at_with_r0), (), block_len))
        };
        let ((mut N_0t, mut M_0t_at_with_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                N_0t,
                M_0t_at_with_r0,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2062:1"]
#[derive(Clone, Debug)]
pub struct instructionVar103 {
    M_0t: M_0t,
    N_0t_at_with_r0: N_0t_at_with_r0,
}
impl instructionVar103 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            M_0t,
            N_0t_at_with_r0,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_with_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_with_r0 = if let Some((len, table)) =
                N_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_with_r0, M_0t), (), block_len))
        };
        let ((mut N_0t_at_with_r0, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                M_0t,
                N_0t_at_with_r0,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2052:1"]
#[derive(Clone, Debug)]
pub struct instructionVar104 {
    M_0t: M_0t,
    N_0t_at_with_r0: N_0t_at_with_r0,
}
impl instructionVar104 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            M_0t,
            N_0t_at_with_r0,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_with_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_with_r0 = if let Some((len, table)) =
                N_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (5u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t_at_with_r0), (), block_len))
        };
        let ((mut M_0t, mut N_0t_at_with_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                M_0t,
                N_0t_at_with_r0,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2042:1"]
#[derive(Clone, Debug)]
pub struct instructionVar105 {
    N_0t_at_with_r0: N_0t_at_with_r0,
    M_0t: M_0t,
}
impl instructionVar105 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            N_0t_at_with_r0,
            M_0t,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_with_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_with_r0 = if let Some((len, table)) =
                N_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_with_r0, M_0t), (), block_len))
        };
        let ((mut N_0t_at_with_r0, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                N_0t_at_with_r0,
                M_0t,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2031:1"]
#[derive(Clone, Debug)]
pub struct instructionVar106 {
    M_0t_at: M_0t_at,
    N_0t: N_0t,
}
impl instructionVar106 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t_at, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at = if let Some((len, table)) = M_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at, N_0t), (), block_len))
        };
        let ((mut M_0t_at, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t_at, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2020:1"]
#[derive(Clone, Debug)]
pub struct instructionVar107 {
    N_0t: N_0t,
    M_0t_at: M_0t_at,
}
impl instructionVar107 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t_at } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at = if let Some((len, table)) = M_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (5u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at, N_0t), (), block_len))
        };
        let ((mut M_0t_at, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t_at }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2009:1"]
#[derive(Clone, Debug)]
pub struct instructionVar108 {
    N_0t: N_0t,
    M_0t_at: M_0t_at,
}
impl instructionVar108 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t_at } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at = if let Some((len, table)) = M_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t_at), (), block_len))
        };
        let ((mut N_0t, mut M_0t_at), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t_at }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1998:1"]
#[derive(Clone, Debug)]
pub struct instructionVar109 {
    N_0t_at_neg: N_0t_at_neg,
    M_0t: M_0t,
}
impl instructionVar109 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at_neg, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t_at_neg), (), block_len))
        };
        let ((mut M_0t, mut N_0t_at_neg), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at_neg, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1987:1"]
#[derive(Clone, Debug)]
pub struct instructionVar110 {
    M_0t: M_0t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar110 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t_at_neg } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (5u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_neg, M_0t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t_at_neg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1976:1"]
#[derive(Clone, Debug)]
pub struct instructionVar111 {
    M_0t: M_0t,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar111 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t_at_neg } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_neg, M_0t), (), block_len))
        };
        let ((mut N_0t_at_neg, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t_at_neg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1966:1"]
#[derive(Clone, Debug)]
pub struct instructionVar112 {
    N_0t: N_0t,
    M_0t_at1: M_0t_at1,
}
impl instructionVar112 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at1.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at1 = if let Some((len, table)) = M_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at1, N_0t), (), block_len))
        };
        let ((mut M_0t_at1, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1956:1"]
#[derive(Clone, Debug)]
pub struct instructionVar113 {
    N_0t: N_0t,
    M_0t_at1: M_0t_at1,
}
impl instructionVar113 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at1.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at1 = if let Some((len, table)) = M_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t_at1), (), block_len))
        };
        let ((mut N_0t, mut M_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1946:1"]
#[derive(Clone, Debug)]
pub struct instructionVar114 {
    N_0t: N_0t,
    M_0t_at1: M_0t_at1,
}
impl instructionVar114 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at1.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t_at1 = if let Some((len, table)) = M_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at1, N_0t), (), block_len))
        };
        let ((mut M_0t_at1, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1936:1"]
#[derive(Clone, Debug)]
pub struct instructionVar115 {
    M_0t: M_0t,
    N_0t_at1: N_0t_at1,
}
impl instructionVar115 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at1, M_0t), (), block_len))
        };
        let ((mut N_0t_at1, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1926:1"]
#[derive(Clone, Debug)]
pub struct instructionVar116 {
    M_0t: M_0t,
    N_0t_at1: N_0t_at1,
}
impl instructionVar116 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t_at1), (), block_len))
        };
        let ((mut M_0t, mut N_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1916:1"]
#[derive(Clone, Debug)]
pub struct instructionVar117 {
    M_0t: M_0t,
    N_0t_at1: N_0t_at1,
}
impl instructionVar117 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t_at1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t_at1), (), block_len))
        };
        let ((mut M_0t, mut N_0t_at1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t_at1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1906:1"]
#[derive(Clone, Debug)]
pub struct instructionVar118 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar118 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("mov"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1885:1"]
#[derive(Clone, Debug)]
pub struct instructionVar119 {
    M_0t_at: M_0t_at,
    N_0t_at: N_0t_at,
}
impl instructionVar119 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t_at, N_0t_at } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mac.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_1() != (15u64 as i64) as u8 {
                return None;
            }
            let M_0t_at = if let Some((len, table)) = M_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let N_0t_at = if let Some((len, table)) = N_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((M_0t_at, N_0t_at), (), block_len))
        };
        let ((mut M_0t_at, mut N_0t_at), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t_at, N_0t_at }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1863:1"]
#[derive(Clone, Debug)]
pub struct instructionVar120 {
    N_0t_at: N_0t_at,
    M_0t_at: M_0t_at,
}
impl instructionVar120 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at, M_0t_at } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mac.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_1() != (15u64 as i64) as u8 {
                return None;
            }
            let M_0t_at = if let Some((len, table)) = M_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let N_0t_at = if let Some((len, table)) = N_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((M_0t_at, N_0t_at), (), block_len))
        };
        let ((mut M_0t_at, mut N_0t_at), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at, M_0t_at }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1502:1"]
#[derive(Clone, Debug)]
pub struct instructionVar121 {
    FRN_0t: FRN_0t,
    FRM_0t: FRM_0t,
}
impl instructionVar121 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t, FRM_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fsub"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRM_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FRM_0t = if let Some((len, table)) = FRM_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRM_1t = if let Some((len, table)) = DRM_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (1u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((DRM_1t, FRM_0t, DRN_1t, FRN_0t), (), block_len))
        };
        let ((mut DRM_1t, mut FRM_0t, mut DRN_1t, mut FRN_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FRM_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t, FRM_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1413:1"]
#[derive(Clone, Debug)]
pub struct instructionVar122 {
    FRN_0t: FRN_0t,
    FRM_0t: FRM_0t,
}
impl instructionVar122 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t, FRM_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fmul"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRM_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FRM_0t = if let Some((len, table)) = FRM_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRM_1t = if let Some((len, table)) = DRM_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((DRN_1t, FRM_0t, FRN_0t, DRM_1t), (), block_len))
        };
        let ((mut DRN_1t, mut FRM_0t, mut FRN_0t, mut DRM_1t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FRM_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t, FRM_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1287:1"]
#[derive(Clone, Debug)]
pub struct instructionVar123 {
    FR0_t: FR0_t,
    FRM_0t: FRM_0t,
    FRN_0t: FRN_0t,
}
impl instructionVar123 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            FR0_t,
            FRM_0t,
            FRN_0t,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fmac"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FR0_t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FRM_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FRM_0t = if let Some((len, table)) = FRM_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (14u64 as i64) as u8 {
                return None;
            }
            let FR0_t = if let Some((len, table)) = FR0_t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((FR0_t, FRN_0t, FRM_0t), (), block_len))
        };
        let ((mut FR0_t, mut FRN_0t, mut FRM_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FR0_t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FRM_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                FR0_t,
                FRM_0t,
                FRN_0t,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1200:1"]
#[derive(Clone, Debug)]
pub struct instructionVar124 {
    FRM_0t: FRM_0t,
    FRN_0t: FRN_0t,
}
impl instructionVar124 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRM_0t, FRN_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fdiv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRM_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FRM_0t = if let Some((len, table)) = FRM_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRM_1t = if let Some((len, table)) = DRM_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRM_0t, FRN_0t, DRM_1t, DRN_1t), (), block_len))
        };
        let ((mut FRM_0t, mut FRN_0t, mut DRM_1t, mut DRN_1t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRM_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRM_0t, FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1158:1"]
#[derive(Clone, Debug)]
pub struct instructionVar125 {
    FRN_0t: FRN_0t,
    FRM_0t: FRM_0t,
}
impl instructionVar125 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t, FRM_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fcmp"),
            DisplayElement::Literal("/gt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRM_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRM_0t = if let Some((len, table)) = FRM_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRM_1t = if let Some((len, table)) = DRM_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (5u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRM_0t, DRN_1t, DRM_1t, FRN_0t), (), block_len))
        };
        let ((mut FRM_0t, mut DRN_1t, mut DRM_1t, mut FRN_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FRM_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t, FRM_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1104:1"]
#[derive(Clone, Debug)]
pub struct instructionVar126 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar126 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("extu.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (13u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1094:1"]
#[derive(Clone, Debug)]
pub struct instructionVar127 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar127 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("extu.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (12u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1084:1"]
#[derive(Clone, Debug)]
pub struct instructionVar128 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar128 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("exts.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (15u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1075:1"]
#[derive(Clone, Debug)]
pub struct instructionVar129 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar129 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("exts.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (6u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (14u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1054:1"]
#[derive(Clone, Debug)]
pub struct instructionVar130 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar130 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("dmulu.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_1() != (5u64 as i64) as u8 {
                return None;
            }
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1043:1"]
#[derive(Clone, Debug)]
pub struct instructionVar131 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar131 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("dmuls.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_1() != (13u64 as i64) as u8 {
                return None;
            }
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:929:1"]
#[derive(Clone, Debug)]
pub struct instructionVar132 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar132 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("div1"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_1() != (4u64 as i64) as u8 {
                return None;
            }
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:905:1"]
#[derive(Clone, Debug)]
pub struct instructionVar133 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar133 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("div0s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:879:1"]
#[derive(Clone, Debug)]
pub struct instructionVar134 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar134 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/str"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (12u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:850:1"]
#[derive(Clone, Debug)]
pub struct instructionVar135 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar135 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/hs"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:840:1"]
#[derive(Clone, Debug)]
pub struct instructionVar136 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar136 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/hi"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:830:1"]
#[derive(Clone, Debug)]
pub struct instructionVar137 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar137 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/gt"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:821:1"]
#[derive(Clone, Debug)]
pub struct instructionVar138 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar138 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/ge"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:811:1"]
#[derive(Clone, Debug)]
pub struct instructionVar139 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar139 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:662:1"]
#[derive(Clone, Debug)]
pub struct instructionVar140 {
    N_0t: N_0t,
    M_0t: M_0t,
}
impl instructionVar140 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, M_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("and"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (2u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (9u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, M_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:651:1"]
#[derive(Clone, Debug)]
pub struct instructionVar141 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar141 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("addv"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (15u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:638:1"]
#[derive(Clone, Debug)]
pub struct instructionVar142 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar142 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("addc"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (14u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t, M_0t), (), block_len))
        };
        let ((mut N_0t, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:618:1"]
#[derive(Clone, Debug)]
pub struct instructionVar143 {
    M_0t: M_0t,
    N_0t: N_0t,
}
impl instructionVar143 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("add"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (3u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (12u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t, N_0t), (), block_len))
        };
        let ((mut M_0t, mut N_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2967:1"]
#[derive(Clone, Debug)]
pub struct instructionVar144 {}
impl instructionVar144 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("synco")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (171u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2636:1"]
#[derive(Clone, Debug)]
pub struct instructionVar145 {}
impl instructionVar145 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("sleep")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (27u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2494:1"]
#[derive(Clone, Debug)]
pub struct instructionVar146 {}
impl instructionVar146 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("sett")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (24u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2484:1"]
#[derive(Clone, Debug)]
pub struct instructionVar147 {}
impl instructionVar147 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("sets")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (88u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2472:1"]
#[derive(Clone, Debug)]
pub struct instructionVar148 {}
impl instructionVar148 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("rts")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (11u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2458:1"]
#[derive(Clone, Debug)]
pub struct instructionVar149 {}
impl instructionVar149 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("rte")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (43u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2334:1"]
#[derive(Clone, Debug)]
pub struct instructionVar150 {}
impl instructionVar150 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nop")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (9u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1856:1"]
#[derive(Clone, Debug)]
pub struct instructionVar151 {}
impl instructionVar151 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("ldtlb")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (56u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1469:1"]
#[derive(Clone, Debug)]
pub struct instructionVar152 {}
impl instructionVar152 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fpchg")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (63485u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1457:1"]
#[derive(Clone, Debug)]
pub struct instructionVar153 {}
impl instructionVar153 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("fschg")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (62461u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1443:1"]
#[derive(Clone, Debug)]
pub struct instructionVar154 {}
impl instructionVar154 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("frchg")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (64509u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:917:1"]
#[derive(Clone, Debug)]
pub struct instructionVar155 {}
impl instructionVar155 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("div0u")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (25u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:801:1"]
#[derive(Clone, Debug)]
pub struct instructionVar156 {}
impl instructionVar156 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("clrt")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (8u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:791:1"]
#[derive(Clone, Debug)]
pub struct instructionVar157 {}
impl instructionVar157 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("clrs")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (72u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:780:1"]
#[derive(Clone, Debug)]
pub struct instructionVar158 {}
impl instructionVar158 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("clrmac")];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_3() != (40u64 as i64) as u16 {
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:3046:1"]
#[derive(Clone, Debug)]
pub struct instructionVar159 {
    U_0t1: U_0t1,
}
impl instructionVar159 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("xor.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (206u64 as i64) as u8 {
                return None;
            }
            let U_0t1 = if let Some((len, table)) = U_0t1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t1), (), block_len))
        };
        let ((mut U_0t1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:3036:1"]
#[derive(Clone, Debug)]
pub struct instructionVar160 {
    U_0t_r0: U_0t_r0,
}
impl instructionVar160 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_r0 } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("xor"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        U_0t_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (202u64 as i64) as u8 {
                return None;
            }
            let U_0t_r0 = if let Some((len, table)) = U_0t_r0::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_r0), (), block_len))
        };
        let ((mut U_0t_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:3017:1"]
#[derive(Clone, Debug)]
pub struct instructionVar161 {
    U_0t1: U_0t1,
}
impl instructionVar161 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("tst.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (204u64 as i64) as u8 {
                return None;
            }
            let U_0t1 = if let Some((len, table)) = U_0t1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t1), (), block_len))
        };
        let ((mut U_0t1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:3007:1"]
#[derive(Clone, Debug)]
pub struct instructionVar162 {
    U_0t_r0: U_0t_r0,
}
impl instructionVar162 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_r0 } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("tst"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        U_0t_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (200u64 as i64) as u8 {
                return None;
            }
            let U_0t_r0 = if let Some((len, table)) = U_0t_r0::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_r0), (), block_len))
        };
        let ((mut U_0t_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2990:1"]
#[derive(Clone, Debug)]
pub struct instructionVar163 {
    U_0t: U_0t,
}
impl instructionVar163 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("trapa"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (195u64 as i64) as u8 {
                return None;
            }
            let U_0t = if let Some((len, table)) = U_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t), (), block_len))
        };
        let ((mut U_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2396:1"]
#[derive(Clone, Debug)]
pub struct instructionVar164 {
    U_0t1: U_0t1,
}
impl instructionVar164 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("or.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (207u64 as i64) as u8 {
                return None;
            }
            let U_0t1 = if let Some((len, table)) = U_0t1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t1), (), block_len))
        };
        let ((mut U_0t1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2386:1"]
#[derive(Clone, Debug)]
pub struct instructionVar165 {
    U_0t_r0: U_0t_r0,
}
impl instructionVar165 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_r0 } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("or"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        U_0t_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (203u64 as i64) as u8 {
                return None;
            }
            let U_0t_r0 = if let Some((len, table)) = U_0t_r0::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_r0), (), block_len))
        };
        let ((mut U_0t_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2252:1"]
#[derive(Clone, Debug)]
pub struct instructionVar166 {
    U_0t_4pc: U_0t_4pc,
}
impl instructionVar166 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_4pc } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mova"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t_4pc.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (199u64 as i64) as u8 {
                return None;
            }
            let U_0t_4pc = if let Some((len, table)) = U_0t_4pc::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_4pc), (), block_len))
        };
        let ((mut U_0t_4pc), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_4pc.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_4pc }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2232:1"]
#[derive(Clone, Debug)]
pub struct instructionVar167 {
    U_2t_M0_dispr02: U_2t_M0_dispr02,
}
impl instructionVar167 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_2t_M0_dispr02 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_2t_M0_dispr02.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (133u64 as i64) as u8 {
                return None;
            }
            let U_2t_M0_dispr02 = if let Some((len, table)) =
                U_2t_M0_dispr02::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_2t_M0_dispr02, M_0t), (), block_len))
        };
        let ((mut U_2t_M0_dispr02, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_2t_M0_dispr02.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_2t_M0_dispr02 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2222:1"]
#[derive(Clone, Debug)]
pub struct instructionVar168 {
    U_2t_M0_dispr01: U_2t_M0_dispr01,
}
impl instructionVar168 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_2t_M0_dispr01 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_2t_M0_dispr01.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (132u64 as i64) as u8 {
                return None;
            }
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let U_2t_M0_dispr01 = if let Some((len, table)) =
                U_2t_M0_dispr01::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_2t_M0_dispr01, M_0t), (), block_len))
        };
        let ((mut U_2t_M0_dispr01, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_2t_M0_dispr01.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_2t_M0_dispr01 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2202:1"]
#[derive(Clone, Debug)]
pub struct instructionVar169 {
    U_2t_M0_dispr02: U_2t_M0_dispr02,
}
impl instructionVar169 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_2t_M0_dispr02 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        U_2t_M0_dispr02.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (129u64 as i64) as u8 {
                return None;
            }
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let U_2t_M0_dispr02 = if let Some((len, table)) =
                U_2t_M0_dispr02::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((M_0t, U_2t_M0_dispr02), (), block_len))
        };
        let ((mut M_0t, mut U_2t_M0_dispr02), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_2t_M0_dispr02.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_2t_M0_dispr02 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2192:1"]
#[derive(Clone, Debug)]
pub struct instructionVar170 {
    U_2t_M0_dispr01: U_2t_M0_dispr01,
}
impl instructionVar170 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_2t_M0_dispr01 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        U_2t_M0_dispr01.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (128u64 as i64) as u8 {
                return None;
            }
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let U_2t_M0_dispr01 = if let Some((len, table)) =
                U_2t_M0_dispr01::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_2t_M0_dispr01, M_0t), (), block_len))
        };
        let ((mut U_2t_M0_dispr01, mut M_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_2t_M0_dispr01.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_2t_M0_dispr01 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2182:1"]
#[derive(Clone, Debug)]
pub struct instructionVar171 {
    U_0t_gbr_at_4: U_0t_gbr_at_4,
}
impl instructionVar171 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_gbr_at_4 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(", "),
        ];
        display.extend_from_slice(&extend);
        U_0t_gbr_at_4.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (194u64 as i64) as u8 {
                return None;
            }
            let U_0t_gbr_at_4 = if let Some((len, table)) = U_0t_gbr_at_4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_gbr_at_4), (), block_len))
        };
        let ((mut U_0t_gbr_at_4), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_gbr_at_4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_gbr_at_4 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2172:1"]
#[derive(Clone, Debug)]
pub struct instructionVar172 {
    U_0t_gbr_at_2: U_0t_gbr_at_2,
}
impl instructionVar172 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_gbr_at_2 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        U_0t_gbr_at_2.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (193u64 as i64) as u8 {
                return None;
            }
            let U_0t_gbr_at_2 = if let Some((len, table)) = U_0t_gbr_at_2::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_gbr_at_2), (), block_len))
        };
        let ((mut U_0t_gbr_at_2), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_gbr_at_2.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_gbr_at_2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2162:1"]
#[derive(Clone, Debug)]
pub struct instructionVar173 {
    U_0t_gbr_at_1: U_0t_gbr_at_1,
}
impl instructionVar173 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_gbr_at_1 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::r0),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        U_0t_gbr_at_1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (192u64 as i64) as u8 {
                return None;
            }
            let U_0t_gbr_at_1 = if let Some((len, table)) = U_0t_gbr_at_1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_gbr_at_1), (), block_len))
        };
        let ((mut U_0t_gbr_at_1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_gbr_at_1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_gbr_at_1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2152:1"]
#[derive(Clone, Debug)]
pub struct instructionVar174 {
    U_0t_gbr_at_4: U_0t_gbr_at_4,
}
impl instructionVar174 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_gbr_at_4 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t_gbr_at_4.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (198u64 as i64) as u8 {
                return None;
            }
            let U_0t_gbr_at_4 = if let Some((len, table)) = U_0t_gbr_at_4::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_gbr_at_4), (), block_len))
        };
        let ((mut U_0t_gbr_at_4), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_gbr_at_4.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_gbr_at_4 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2142:1"]
#[derive(Clone, Debug)]
pub struct instructionVar175 {
    U_0t_gbr_at_2: U_0t_gbr_at_2,
}
impl instructionVar175 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_gbr_at_2 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t_gbr_at_2.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (197u64 as i64) as u8 {
                return None;
            }
            let U_0t_gbr_at_2 = if let Some((len, table)) = U_0t_gbr_at_2::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_gbr_at_2), (), block_len))
        };
        let ((mut U_0t_gbr_at_2), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_gbr_at_2.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_gbr_at_2 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2132:1"]
#[derive(Clone, Debug)]
pub struct instructionVar176 {
    U_0t_gbr_at_1: U_0t_gbr_at_1,
}
impl instructionVar176 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_gbr_at_1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t_gbr_at_1.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::r0),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (196u64 as i64) as u8 {
                return None;
            }
            let U_0t_gbr_at_1 = if let Some((len, table)) = U_0t_gbr_at_1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_gbr_at_1), (), block_len))
        };
        let ((mut U_0t_gbr_at_1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_gbr_at_1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_gbr_at_1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:895:1"]
#[derive(Clone, Debug)]
pub struct instructionVar177 {
    I_0t_r0: I_0t_r0,
}
impl instructionVar177 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0t_r0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("cmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        I_0t_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (136u64 as i64) as u8 {
                return None;
            }
            let I_0t_r0 = if let Some((len, table)) = I_0t_r0::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((I_0t_r0), (), block_len))
        };
        let ((mut I_0t_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_0t_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_0t_r0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:769:1"]
#[derive(Clone, Debug)]
pub struct instructionVar178 {
    I_0tbranch: I_0tbranch,
}
impl instructionVar178 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0tbranch } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("bt"),
            DisplayElement::Literal("/s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        I_0tbranch.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (141u64 as i64) as u8 {
                return None;
            }
            let I_0tbranch = if let Some((len, table)) = I_0tbranch::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((I_0tbranch), (), block_len))
        };
        let ((mut I_0tbranch), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_0tbranch.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:759:1"]
#[derive(Clone, Debug)]
pub struct instructionVar179 {
    I_0tbranch: I_0tbranch,
}
impl instructionVar179 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0tbranch } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bt"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        I_0tbranch.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (137u64 as i64) as u8 {
                return None;
            }
            let I_0tbranch = if let Some((len, table)) = I_0tbranch::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((I_0tbranch), (), block_len))
        };
        let ((mut I_0tbranch), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_0tbranch.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:700:1"]
#[derive(Clone, Debug)]
pub struct instructionVar180 {
    I_0tbranch: I_0tbranch,
}
impl instructionVar180 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0tbranch } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("bf"),
            DisplayElement::Literal("/s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        I_0tbranch.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (143u64 as i64) as u8 {
                return None;
            }
            let I_0tbranch = if let Some((len, table)) = I_0tbranch::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((I_0tbranch), (), block_len))
        };
        let ((mut I_0tbranch), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_0tbranch.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:690:1"]
#[derive(Clone, Debug)]
pub struct instructionVar181 {
    I_0tbranch: I_0tbranch,
}
impl instructionVar181 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0tbranch } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bf"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        I_0tbranch.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (139u64 as i64) as u8 {
                return None;
            }
            let I_0tbranch = if let Some((len, table)) = I_0tbranch::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((I_0tbranch), (), block_len))
        };
        let ((mut I_0tbranch), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_0tbranch.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_0tbranch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:680:1"]
#[derive(Clone, Debug)]
pub struct instructionVar182 {
    U_0t1: U_0t1,
}
impl instructionVar182 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("and.b"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (205u64 as i64) as u8 {
                return None;
            }
            let U_0t1 = if let Some((len, table)) = U_0t1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t1), (), block_len))
        };
        let ((mut U_0t1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:671:1"]
#[derive(Clone, Debug)]
pub struct instructionVar183 {
    U_0t_r0: U_0t_r0,
}
impl instructionVar183 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { U_0t_r0 } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("and"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        U_0t_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_2() != (201u64 as i64) as u8 {
                return None;
            }
            let U_0t_r0 = if let Some((len, table)) = U_0t_r0::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((U_0t_r0), (), block_len))
        };
        let ((mut U_0t_r0), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        U_0t_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { U_0t_r0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2242:1"]
#[derive(Clone, Debug)]
pub struct instructionVar184 {
    N_0t: N_0t,
    U_2t_M0_dispr04: U_2t_M0_dispr04,
}
impl instructionVar184 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            N_0t,
            U_2t_M0_dispr04,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_2t_M0_dispr04.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (5u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let U_2t_M0_dispr04 = if let Some((len, table)) =
                U_2t_M0_dispr04::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t, M_0t, U_2t_M0_dispr04), (), block_len))
        };
        let ((mut N_0t, mut M_0t, mut U_2t_M0_dispr04), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        U_2t_M0_dispr04.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                N_0t,
                U_2t_M0_dispr04,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2212:1"]
#[derive(Clone, Debug)]
pub struct instructionVar185 {
    M_0t: M_0t,
    U_2t_N0_dispr04: U_2t_N0_dispr04,
}
impl instructionVar185 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            M_0t,
            U_2t_N0_dispr04,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        U_2t_N0_dispr04.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (1u64 as i64) as u8 {
                return None;
            }
            let M_0t = if let Some((len, table)) = M_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let U_2t_N0_dispr04 = if let Some((len, table)) =
                U_2t_N0_dispr04::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t, M_0t, U_2t_N0_dispr04), (), block_len))
        };
        let ((mut N_0t, mut M_0t, mut U_2t_N0_dispr04), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        U_2t_N0_dispr04.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                M_0t,
                U_2t_N0_dispr04,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2122:1"]
#[derive(Clone, Debug)]
pub struct instructionVar186 {
    N_0t: N_0t,
    U_0t_4pc: U_0t_4pc,
}
impl instructionVar186 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, U_0t_4pc } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t_4pc.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (13u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let U_0t_4pc = if let Some((len, table)) = U_0t_4pc::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t, U_0t_4pc), (), block_len))
        };
        let ((mut N_0t, mut U_0t_4pc), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        U_0t_4pc.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, U_0t_4pc }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2112:1"]
#[derive(Clone, Debug)]
pub struct instructionVar187 {
    N_0t: N_0t,
    U_0t_2pc: U_0t_2pc,
}
impl instructionVar187 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, U_0t_2pc } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("mov.w"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        U_0t_2pc.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (9u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let U_0t_2pc = if let Some((len, table)) = U_0t_2pc::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t, U_0t_2pc), (), block_len))
        };
        let ((mut N_0t, mut U_0t_2pc), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        U_0t_2pc.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, U_0t_2pc }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2102:1"]
#[derive(Clone, Debug)]
pub struct instructionVar188 {
    I_0t: I_0t,
    N_0t: N_0t,
}
impl instructionVar188 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0t, N_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("mov"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        I_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (14u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let I_0t = if let Some((len, table)) = I_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t, I_0t), (), block_len))
        };
        let ((mut N_0t, mut I_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_0t, N_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:735:1"]
#[derive(Clone, Debug)]
pub struct instructionVar189 {
    I_1tbranch: I_1tbranch,
}
impl instructionVar189 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_1tbranch } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bsr"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        I_1tbranch.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (11u64 as i64) as u8 {
                return None;
            }
            let I_1tbranch = if let Some((len, table)) = I_1tbranch::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((I_1tbranch), (), block_len))
        };
        let ((mut I_1tbranch), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_1tbranch.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_1tbranch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:712:1"]
#[derive(Clone, Debug)]
pub struct instructionVar190 {
    I_1tbranch: I_1tbranch,
}
impl instructionVar190 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_1tbranch } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("bra"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        I_1tbranch.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (10u64 as i64) as u8 {
                return None;
            }
            let I_1tbranch = if let Some((len, table)) = I_1tbranch::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((I_1tbranch), (), block_len))
        };
        let ((mut I_1tbranch), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        I_1tbranch.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { I_1tbranch }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:628:1"]
#[derive(Clone, Debug)]
pub struct instructionVar191 {
    N_0t: N_0t,
    I_0t: I_0t,
}
impl instructionVar191 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t, I_0t } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("add"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        I_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (7u64 as i64) as u8 {
                return None;
            }
            let N_0t = if let Some((len, table)) = N_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let I_0t = if let Some((len, table)) = I_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            *context = context_current;
            Some(((N_0t, I_0t), (), block_len))
        };
        let ((mut N_0t, mut I_0t), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        I_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t, I_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1428:1"]
#[derive(Clone, Debug)]
pub struct instructionVar192 {
    FRN_0t: FRN_0t,
}
impl instructionVar192 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fneg"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_6 = token_parser.OP_6();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (77u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((FRN_0t, DRN_1t), (OP_6), block_len))
        };
        let ((mut FRN_0t, mut DRN_1t), (OP_6), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1271:1"]
#[derive(Clone, Debug)]
pub struct instructionVar193 {
    FRN_0t: FRN_0t,
}
impl instructionVar193 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0t } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("float"),
            DisplayElement::Literal(" "),
            DisplayElement::Register(Register::FPUL),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_6 = token_parser.OP_6();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_4() != (45u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((DRN_1t, FRN_0t), (OP_6), block_len))
        };
        let ((mut DRN_1t, mut FRN_0t), (OP_6), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:746:1"]
#[derive(Clone, Debug)]
pub struct instructionVar194 {
    N_0: u8,
}
impl instructionVar194 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("bsrf"),
            DisplayElement::Literal(" "),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_4() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:723:1"]
#[derive(Clone, Debug)]
pub struct instructionVar195 {
    N_0: u8,
}
impl instructionVar195 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("braf"),
            DisplayElement::Literal(" "),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_4() != (35u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1114:1"]
#[derive(Clone, Debug)]
pub struct instructionVar196 {
    FRN_0: u8,
}
impl instructionVar196 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fabs"),
            DisplayElement::Literal(" "),
            meaning_94590182964544(usize::try_from(*FRN_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FRN_0 = token_parser.FRN_0();
            let DRN_1 = token_parser.DRN_1();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_4() != (93u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (DRN_1, FRN_0), block_len))
        };
        let ((), (DRN_1, FRN_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { FRN_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1302:1"]
#[derive(Clone, Debug)]
pub struct instructionVar197 {
    FRN_0: u8,
    FRM_0: u8,
}
impl instructionVar197 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRN_0, FRM_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fmov"),
            DisplayElement::Literal(" "),
            meaning_94590182964544(usize::try_from(*FRM_0).unwrap()),
            DisplayElement::Literal(","),
            meaning_94590182964544(usize::try_from(*FRN_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FRN_0 = token_parser.FRN_0();
            let XDRN = token_parser.XDRN();
            let FRM_0 = token_parser.FRM_0();
            let XDRM = token_parser.XDRM();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_1() != (12u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (XDRM, FRN_0, XDRN, FRM_0), block_len))
        };
        let ((), (XDRM, FRN_0, XDRN, FRM_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { FRN_0, FRM_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1129:1"]
#[derive(Clone, Debug)]
pub struct instructionVar198 {
    FRM_0: u8,
    FRN_0: u8,
}
impl instructionVar198 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRM_0, FRN_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("fadd"),
            DisplayElement::Literal(" "),
            meaning_94590182964544(usize::try_from(*FRM_0).unwrap()),
            DisplayElement::Literal(","),
            meaning_94590182964544(usize::try_from(*FRN_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FRN_0 = token_parser.FRN_0();
            let DRN_1 = token_parser.DRN_1();
            let FRM_0 = token_parser.FRM_0();
            let DRM_1 = token_parser.DRM_1();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            if token_parser.OP_1() != (0u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((), (FRN_0, DRN_1, DRM_1, FRM_0), block_len))
        };
        let ((), (FRN_0, DRN_1, DRM_1, FRM_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        *context = context_current;
        Some((inst_len, Self { FRM_0, FRN_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1398:1"]
#[derive(Clone, Debug)]
pub struct instructionVar199 {
    N_0t_at_with_r0: N_0t_at_with_r0,
    FRM_0: u8,
}
impl instructionVar199 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            N_0t_at_with_r0,
            FRM_0,
        } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("fmov"),
            DisplayElement::Literal(" "),
            meaning_94590182964544(usize::try_from(*FRM_0).unwrap()),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        N_0t_at_with_r0.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let XDRM = token_parser.XDRM();
            let FRM_0 = token_parser.FRM_0();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_with_r0 = if let Some((len, table)) =
                N_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_with_r0), (XDRM, FRM_0), block_len))
        };
        let ((mut N_0t_at_with_r0), (XDRM, FRM_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                N_0t_at_with_r0,
                FRM_0,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1383:1"]
#[derive(Clone, Debug)]
pub struct instructionVar200 {
    M_0t_at_with_r0: M_0t_at_with_r0,
    FRN_0: u8,
}
impl instructionVar200 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {
            M_0t_at_with_r0,
            FRN_0,
        } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at_with_r0.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            meaning_94590182964544(usize::try_from(*FRN_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let XDRN = token_parser.XDRN();
            let FRN_0 = token_parser.FRN_0();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let M_0t_at_with_r0 = if let Some((len, table)) =
                M_0t_at_with_r0::parse(
                    tokens,
                    &mut context_current,
                    inst_start,
                    global_set,
                ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (6u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at_with_r0), (XDRN, FRN_0), block_len))
        };
        let ((mut M_0t_at_with_r0), (XDRN, FRN_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t_at_with_r0.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((
            inst_len,
            Self {
                M_0t_at_with_r0,
                FRN_0,
            },
        ))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1366:1"]
#[derive(Clone, Debug)]
pub struct instructionVar201 {
    N_0t_at_neg: N_0t_at_neg,
    FRM_0: u8,
}
impl instructionVar201 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at_neg, FRM_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            meaning_94590182964544(usize::try_from(*FRM_0).unwrap()),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let XDRM = token_parser.XDRM();
            let FRM_0 = token_parser.FRM_0();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (11u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_neg), (XDRM, FRM_0), block_len))
        };
        let ((mut N_0t_at_neg), (XDRM, FRM_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at_neg, FRM_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1349:1"]
#[derive(Clone, Debug)]
pub struct instructionVar202 {
    M_0t_at: M_0t_at,
    FRN_0: u8,
}
impl instructionVar202 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t_at, FRN_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            meaning_94590182964544(usize::try_from(*FRN_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let XDRN = token_parser.XDRN();
            let FRN_0 = token_parser.FRN_0();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let M_0t_at = if let Some((len, table)) = M_0t_at::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (9u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at), (XDRN, FRN_0), block_len))
        };
        let ((mut M_0t_at), (XDRN, FRN_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t_at.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t_at, FRN_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1334:1"]
#[derive(Clone, Debug)]
pub struct instructionVar203 {
    N_0t_at1: N_0t_at1,
    FRM_0: u8,
}
impl instructionVar203 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_at1, FRM_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
            meaning_94590182964544(usize::try_from(*FRM_0).unwrap()),
            DisplayElement::Literal(","),
        ];
        display.extend_from_slice(&extend);
        N_0t_at1.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let FRM_0 = token_parser.FRM_0();
            let XDRM = token_parser.XDRM();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let N_0t_at1 = if let Some((len, table)) = N_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (10u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at1), (FRM_0, XDRM), block_len))
        };
        let ((mut N_0t_at1), (FRM_0, XDRM), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_at1, FRM_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1316:1"]
#[derive(Clone, Debug)]
pub struct instructionVar204 {
    M_0t_at1: M_0t_at1,
    FRN_0: u8,
}
impl instructionVar204 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0t_at1, FRN_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("fmov.s"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        M_0t_at1.display_extend(display, context);
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal(","),
            meaning_94590182964544(usize::try_from(*FRN_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let XDRN = token_parser.XDRN();
            let FRN_0 = token_parser.FRN_0();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let M_0t_at1 = if let Some((len, table)) = M_0t_at1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (8u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((M_0t_at1), (XDRN, FRN_0), block_len))
        };
        let ((mut M_0t_at1), (XDRN, FRN_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        M_0t_at1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { M_0t_at1, FRN_0 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1144:1"]
#[derive(Clone, Debug)]
pub struct instructionVar205 {
    FRM_0t: FRM_0t,
    FRN_0t: FRN_0t,
}
impl instructionVar205 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { FRM_0t, FRN_0t } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Literal("fcmp"),
            DisplayElement::Literal("/eq"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        FRM_0t.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        FRN_0t.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_6 = token_parser.OP_6();
            let OP_10 = token_parser.OP_10();
            if token_parser.OP_0() != (15u64 as i64) as u8 {
                return None;
            }
            let FRN_0t = if let Some((len, table)) = FRN_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRN_1t = if let Some((len, table)) = DRN_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let FRM_0t = if let Some((len, table)) = FRM_0t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            let DRM_1t = if let Some((len, table)) = DRM_1t::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (4u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((DRN_1t, FRM_0t, DRM_1t, FRN_0t), (OP_6, OP_10), block_len))
        };
        let (
            (mut DRN_1t, mut FRM_0t, mut DRM_1t, mut FRN_0t),
            (OP_6, OP_10),
            block_len,
        ) = block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        FRM_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        FRN_0t.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { FRM_0t, FRN_0t }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2793:1"]
#[derive(Clone, Debug)]
pub struct instructionVar206 {
    BANKt: BANKt,
    N_0t_at_neg: N_0t_at_neg,
}
impl instructionVar206 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { BANKt, N_0t_at_neg } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("stc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        BANKt.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_at_neg.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_at_neg = if let Some((len, table)) = N_0t_at_neg::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_9() != (1u64 as i64) as u8 {
                return None;
            }
            let BANKt = if let Some((len, table)) = BANKt::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (3u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((N_0t_at_neg, BANKt), (), block_len))
        };
        let ((mut N_0t_at_neg, mut BANKt), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        BANKt.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_at_neg.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { BANKt, N_0t_at_neg }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:2710:1"]
#[derive(Clone, Debug)]
pub struct instructionVar207 {
    N_0t_bank: N_0t_bank,
    BANKt: BANKt,
}
impl instructionVar207 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_bank, BANKt } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("stc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        BANKt.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        N_0t_bank.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (0u64 as i64) as u8 {
                return None;
            }
            let N_0t_bank = if let Some((len, table)) = N_0t_bank::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_9() != (1u64 as i64) as u8 {
                return None;
            }
            let BANKt = if let Some((len, table)) = BANKt::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (2u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((BANKt, N_0t_bank), (), block_len))
        };
        let ((mut BANKt, mut N_0t_bank), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_bank.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        BANKt.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_bank, BANKt }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1735:1"]
#[derive(Clone, Debug)]
pub struct instructionVar208 {
    BANKt: BANKt,
    N_0t_bank1: N_0t_bank1,
}
impl instructionVar208 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { BANKt, N_0t_bank1 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("ldc.l"),
            DisplayElement::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        N_0t_bank1.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        BANKt.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_bank1 = if let Some((len, table)) = N_0t_bank1::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_9() != (1u64 as i64) as u8 {
                return None;
            }
            let BANKt = if let Some((len, table)) = BANKt::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (7u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((BANKt, N_0t_bank1), (), block_len))
        };
        let ((mut BANKt, mut N_0t_bank1), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        BANKt.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        N_0t_bank1.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { BANKt, N_0t_bank1 }))
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:1662:1"]
#[derive(Clone, Debug)]
pub struct instructionVar209 {
    N_0t_bank: N_0t_bank,
    BANKt: BANKt,
}
impl instructionVar209 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0t_bank, BANKt } = self;
        let extend: [DisplayElement; 2usize] =
            [DisplayElement::Literal("ldc"), DisplayElement::Literal(" ")];
        display.extend_from_slice(&extend);
        N_0t_bank.display_extend(display, context);
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal(",")];
        display.extend_from_slice(&extend);
        BANKt.display_extend(display, context);
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            if token_parser.OP_0() != (4u64 as i64) as u8 {
                return None;
            }
            let N_0t_bank = if let Some((len, table)) = N_0t_bank::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_9() != (1u64 as i64) as u8 {
                return None;
            }
            let BANKt = if let Some((len, table)) = BANKt::parse(
                tokens,
                &mut context_current,
                inst_start,
                global_set,
            ) {
                block_len = len as u32;
                table
            } else {
                return None;
            };
            if token_parser.OP_1() != (14u64 as i64) as u8 {
                return None;
            }
            *context = context_current;
            Some(((BANKt, N_0t_bank), (), block_len))
        };
        let ((mut BANKt, mut N_0t_bank), (), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let inst_next = inst_start + inst_len;
        N_0t_bank.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        BANKt.disassembly(
            &mut context_current,
            inst_start,
            inst_next,
            global_set,
        );
        *context = context_current;
        Some((inst_len, Self { N_0t_bank, BANKt }))
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
            Self::Var51(x) => x.display_extend(display, context),
            Self::Var52(x) => x.display_extend(display, context),
            Self::Var53(x) => x.display_extend(display, context),
            Self::Var54(x) => x.display_extend(display, context),
            Self::Var55(x) => x.display_extend(display, context),
            Self::Var56(x) => x.display_extend(display, context),
            Self::Var57(x) => x.display_extend(display, context),
            Self::Var58(x) => x.display_extend(display, context),
            Self::Var59(x) => x.display_extend(display, context),
            Self::Var60(x) => x.display_extend(display, context),
            Self::Var61(x) => x.display_extend(display, context),
            Self::Var62(x) => x.display_extend(display, context),
            Self::Var63(x) => x.display_extend(display, context),
            Self::Var64(x) => x.display_extend(display, context),
            Self::Var65(x) => x.display_extend(display, context),
            Self::Var66(x) => x.display_extend(display, context),
            Self::Var67(x) => x.display_extend(display, context),
            Self::Var68(x) => x.display_extend(display, context),
            Self::Var69(x) => x.display_extend(display, context),
            Self::Var70(x) => x.display_extend(display, context),
            Self::Var71(x) => x.display_extend(display, context),
            Self::Var72(x) => x.display_extend(display, context),
            Self::Var73(x) => x.display_extend(display, context),
            Self::Var74(x) => x.display_extend(display, context),
            Self::Var75(x) => x.display_extend(display, context),
            Self::Var76(x) => x.display_extend(display, context),
            Self::Var77(x) => x.display_extend(display, context),
            Self::Var78(x) => x.display_extend(display, context),
            Self::Var79(x) => x.display_extend(display, context),
            Self::Var80(x) => x.display_extend(display, context),
            Self::Var81(x) => x.display_extend(display, context),
            Self::Var82(x) => x.display_extend(display, context),
            Self::Var83(x) => x.display_extend(display, context),
            Self::Var84(x) => x.display_extend(display, context),
            Self::Var85(x) => x.display_extend(display, context),
            Self::Var86(x) => x.display_extend(display, context),
            Self::Var87(x) => x.display_extend(display, context),
            Self::Var88(x) => x.display_extend(display, context),
            Self::Var89(x) => x.display_extend(display, context),
            Self::Var90(x) => x.display_extend(display, context),
            Self::Var91(x) => x.display_extend(display, context),
            Self::Var92(x) => x.display_extend(display, context),
            Self::Var93(x) => x.display_extend(display, context),
            Self::Var94(x) => x.display_extend(display, context),
            Self::Var95(x) => x.display_extend(display, context),
            Self::Var96(x) => x.display_extend(display, context),
            Self::Var97(x) => x.display_extend(display, context),
            Self::Var98(x) => x.display_extend(display, context),
            Self::Var99(x) => x.display_extend(display, context),
            Self::Var100(x) => x.display_extend(display, context),
            Self::Var101(x) => x.display_extend(display, context),
            Self::Var102(x) => x.display_extend(display, context),
            Self::Var103(x) => x.display_extend(display, context),
            Self::Var104(x) => x.display_extend(display, context),
            Self::Var105(x) => x.display_extend(display, context),
            Self::Var106(x) => x.display_extend(display, context),
            Self::Var107(x) => x.display_extend(display, context),
            Self::Var108(x) => x.display_extend(display, context),
            Self::Var109(x) => x.display_extend(display, context),
            Self::Var110(x) => x.display_extend(display, context),
            Self::Var111(x) => x.display_extend(display, context),
            Self::Var112(x) => x.display_extend(display, context),
            Self::Var113(x) => x.display_extend(display, context),
            Self::Var114(x) => x.display_extend(display, context),
            Self::Var115(x) => x.display_extend(display, context),
            Self::Var116(x) => x.display_extend(display, context),
            Self::Var117(x) => x.display_extend(display, context),
            Self::Var118(x) => x.display_extend(display, context),
            Self::Var119(x) => x.display_extend(display, context),
            Self::Var120(x) => x.display_extend(display, context),
            Self::Var121(x) => x.display_extend(display, context),
            Self::Var122(x) => x.display_extend(display, context),
            Self::Var123(x) => x.display_extend(display, context),
            Self::Var124(x) => x.display_extend(display, context),
            Self::Var125(x) => x.display_extend(display, context),
            Self::Var126(x) => x.display_extend(display, context),
            Self::Var127(x) => x.display_extend(display, context),
            Self::Var128(x) => x.display_extend(display, context),
            Self::Var129(x) => x.display_extend(display, context),
            Self::Var130(x) => x.display_extend(display, context),
            Self::Var131(x) => x.display_extend(display, context),
            Self::Var132(x) => x.display_extend(display, context),
            Self::Var133(x) => x.display_extend(display, context),
            Self::Var134(x) => x.display_extend(display, context),
            Self::Var135(x) => x.display_extend(display, context),
            Self::Var136(x) => x.display_extend(display, context),
            Self::Var137(x) => x.display_extend(display, context),
            Self::Var138(x) => x.display_extend(display, context),
            Self::Var139(x) => x.display_extend(display, context),
            Self::Var140(x) => x.display_extend(display, context),
            Self::Var141(x) => x.display_extend(display, context),
            Self::Var142(x) => x.display_extend(display, context),
            Self::Var143(x) => x.display_extend(display, context),
            Self::Var144(x) => x.display_extend(display, context),
            Self::Var145(x) => x.display_extend(display, context),
            Self::Var146(x) => x.display_extend(display, context),
            Self::Var147(x) => x.display_extend(display, context),
            Self::Var148(x) => x.display_extend(display, context),
            Self::Var149(x) => x.display_extend(display, context),
            Self::Var150(x) => x.display_extend(display, context),
            Self::Var151(x) => x.display_extend(display, context),
            Self::Var152(x) => x.display_extend(display, context),
            Self::Var153(x) => x.display_extend(display, context),
            Self::Var154(x) => x.display_extend(display, context),
            Self::Var155(x) => x.display_extend(display, context),
            Self::Var156(x) => x.display_extend(display, context),
            Self::Var157(x) => x.display_extend(display, context),
            Self::Var158(x) => x.display_extend(display, context),
            Self::Var159(x) => x.display_extend(display, context),
            Self::Var160(x) => x.display_extend(display, context),
            Self::Var161(x) => x.display_extend(display, context),
            Self::Var162(x) => x.display_extend(display, context),
            Self::Var163(x) => x.display_extend(display, context),
            Self::Var164(x) => x.display_extend(display, context),
            Self::Var165(x) => x.display_extend(display, context),
            Self::Var166(x) => x.display_extend(display, context),
            Self::Var167(x) => x.display_extend(display, context),
            Self::Var168(x) => x.display_extend(display, context),
            Self::Var169(x) => x.display_extend(display, context),
            Self::Var170(x) => x.display_extend(display, context),
            Self::Var171(x) => x.display_extend(display, context),
            Self::Var172(x) => x.display_extend(display, context),
            Self::Var173(x) => x.display_extend(display, context),
            Self::Var174(x) => x.display_extend(display, context),
            Self::Var175(x) => x.display_extend(display, context),
            Self::Var176(x) => x.display_extend(display, context),
            Self::Var177(x) => x.display_extend(display, context),
            Self::Var178(x) => x.display_extend(display, context),
            Self::Var179(x) => x.display_extend(display, context),
            Self::Var180(x) => x.display_extend(display, context),
            Self::Var181(x) => x.display_extend(display, context),
            Self::Var182(x) => x.display_extend(display, context),
            Self::Var183(x) => x.display_extend(display, context),
            Self::Var184(x) => x.display_extend(display, context),
            Self::Var185(x) => x.display_extend(display, context),
            Self::Var186(x) => x.display_extend(display, context),
            Self::Var187(x) => x.display_extend(display, context),
            Self::Var188(x) => x.display_extend(display, context),
            Self::Var189(x) => x.display_extend(display, context),
            Self::Var190(x) => x.display_extend(display, context),
            Self::Var191(x) => x.display_extend(display, context),
            Self::Var192(x) => x.display_extend(display, context),
            Self::Var193(x) => x.display_extend(display, context),
            Self::Var194(x) => x.display_extend(display, context),
            Self::Var195(x) => x.display_extend(display, context),
            Self::Var196(x) => x.display_extend(display, context),
            Self::Var197(x) => x.display_extend(display, context),
            Self::Var198(x) => x.display_extend(display, context),
            Self::Var199(x) => x.display_extend(display, context),
            Self::Var200(x) => x.display_extend(display, context),
            Self::Var201(x) => x.display_extend(display, context),
            Self::Var202(x) => x.display_extend(display, context),
            Self::Var203(x) => x.display_extend(display, context),
            Self::Var204(x) => x.display_extend(display, context),
            Self::Var205(x) => x.display_extend(display, context),
            Self::Var206(x) => x.display_extend(display, context),
            Self::Var207(x) => x.display_extend(display, context),
            Self::Var208(x) => x.display_extend(display, context),
            Self::Var209(x) => x.display_extend(display, context),
        }
    }
    pub fn parse<'a, T>(
        tokens_param: &'a [u8],
        context_param: &mut T,
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
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
        if let Some((inst_next, parsed)) = instructionVar51::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var51(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar52::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var52(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar53::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var53(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar54::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var54(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar55::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var55(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar56::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var56(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar57::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var57(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar58::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var58(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar59::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var59(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar60::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var60(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar61::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var61(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar62::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var62(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar63::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var63(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar64::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var64(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar65::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var65(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar66::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var66(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar67::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var67(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar68::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var68(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar69::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var69(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar70::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var70(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar71::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var71(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar72::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var72(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar73::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var73(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar74::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var74(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar75::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var75(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar76::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var76(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar77::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var77(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar78::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var78(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar79::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var79(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar80::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var80(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar81::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var81(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar82::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var82(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar83::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var83(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar84::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var84(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar85::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var85(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar86::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var86(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar87::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var87(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar88::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var88(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar89::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var89(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar90::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var90(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar91::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var91(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar92::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var92(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar93::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var93(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar94::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var94(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar95::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var95(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar96::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var96(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar97::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var97(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar98::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var98(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar99::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var99(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar100::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var100(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar101::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var101(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar102::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var102(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar103::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var103(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar104::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var104(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar105::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var105(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar106::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var106(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar107::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var107(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar108::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var108(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar109::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var109(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar110::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var110(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar111::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var111(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar112::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var112(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar113::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var113(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar114::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var114(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar115::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var115(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar116::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var116(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar117::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var117(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar118::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var118(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar119::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var119(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar120::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var120(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar121::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var121(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar122::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var122(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar123::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var123(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar124::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var124(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar125::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var125(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar126::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var126(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar127::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var127(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar128::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var128(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar129::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var129(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar130::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var130(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar131::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var131(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar132::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var132(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar133::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var133(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar134::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var134(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar135::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var135(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar136::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var136(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar137::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var137(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar138::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var138(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar139::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var139(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar140::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var140(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar141::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var141(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar142::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var142(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar143::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var143(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar144::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var144(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar145::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var145(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar146::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var146(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar147::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var147(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar148::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var148(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar149::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var149(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar150::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var150(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar151::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var151(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar152::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var152(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar153::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var153(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar154::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var154(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar155::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var155(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar156::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var156(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar157::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var157(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar158::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var158(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar159::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var159(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar160::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var160(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar161::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var161(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar162::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var162(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar163::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var163(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar164::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var164(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar165::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var165(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar166::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var166(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar167::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var167(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar168::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var168(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar169::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var169(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar170::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var170(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar171::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var171(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar172::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var172(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar173::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var173(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar174::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var174(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar175::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var175(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar176::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var176(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar177::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var177(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar178::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var178(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar179::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var179(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar180::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var180(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar181::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var181(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar182::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var182(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar183::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var183(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar184::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var184(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar185::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var185(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar186::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var186(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar187::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var187(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar188::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var188(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar189::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var189(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar190::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var190(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar191::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var191(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar192::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var192(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar193::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var193(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar194::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var194(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar195::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var195(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar196::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var196(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar197::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var197(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar198::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var198(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar199::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var199(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar200::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var200(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar201::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var201(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar202::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var202(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar203::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var203(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar204::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var204(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar205::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var205(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar206::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var206(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar207::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var207(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar208::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var208(parsed)));
        }
        if let Some((inst_next, parsed)) = instructionVar209::parse(
            tokens_param,
            &mut context_current,
            inst_start,
            global_set_param,
        ) {
            *context_param = context_current;
            return Some((inst_next, Self::Var209(parsed)));
        }
        None
    }
}
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:448:1"]
#[derive(Clone, Debug)]
pub struct N_0tjmpVar0 {
    N_0: u8,
}
impl N_0tjmpVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0tjmp"]
#[derive(Clone, Debug)]
pub enum N_0tjmp {
    Var0(N_0tjmpVar0),
}
impl N_0tjmp {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0tjmpVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:506:1"]
#[derive(Clone, Debug)]
pub struct N_0t_sr1Var0 {
    N_0: u8,
}
impl N_0t_sr1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::SR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_sr1"]
#[derive(Clone, Debug)]
pub enum N_0t_sr1 {
    Var0(N_0t_sr1Var0),
}
impl N_0t_sr1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_sr1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:487:1"]
#[derive(Clone, Debug)]
pub struct sgr_tVar0 {}
impl sgr_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::SGR)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table sgr_t"]
#[derive(Clone, Debug)]
pub enum sgr_t {
    Var0(sgr_tVar0),
}
impl sgr_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = sgr_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:529:1"]
#[derive(Clone, Debug)]
pub struct fpul_tVar0 {}
impl fpul_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::FPUL)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table fpul_t"]
#[derive(Clone, Debug)]
pub enum fpul_t {
    Var0(fpul_tVar0),
}
impl fpul_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = fpul_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:521:1"]
#[derive(Clone, Debug)]
pub struct FR0_tVar0 {}
impl FR0_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::fr0)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table FR0_t"]
#[derive(Clone, Debug)]
pub enum FR0_t {
    Var0(FR0_tVar0),
}
impl FR0_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = FR0_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:547:1"]
#[derive(Clone, Debug)]
pub struct N_0t_prVar0 {
    N_0: u8,
}
impl N_0t_prVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::PR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_pr"]
#[derive(Clone, Debug)]
pub enum N_0t_pr {
    Var0(N_0t_prVar0),
}
impl N_0t_pr {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_prVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:549:1"]
#[derive(Clone, Debug)]
pub struct N_0t_fpulVar0 {
    N_0: u8,
}
impl N_0t_fpulVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_fpul"]
#[derive(Clone, Debug)]
pub enum N_0t_fpul {
    Var0(N_0t_fpulVar0),
}
impl N_0t_fpul {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_fpulVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:559:1"]
#[derive(Clone, Debug)]
pub struct N_0t_fpul1Var0 {
    N_0: u8,
}
impl N_0t_fpul1Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 4usize] = [
            DisplayElement::Literal("@"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal("+,"),
            DisplayElement::Register(Register::FPUL),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_fpul1"]
#[derive(Clone, Debug)]
pub enum N_0t_fpul1 {
    Var0(N_0t_fpul1Var0),
}
impl N_0t_fpul1 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_fpul1Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:499:1"]
#[derive(Clone, Debug)]
pub struct N_0t_spcVar0 {
    N_0: u8,
}
impl N_0t_spcVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::SPC),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_spc"]
#[derive(Clone, Debug)]
pub enum N_0t_spc {
    Var0(N_0t_spcVar0),
}
impl N_0t_spc {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_spcVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:523:1"]
#[derive(Clone, Debug)]
pub struct XMTRX_tVar0 {}
impl XMTRX_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Literal("xmtrx")];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table XMTRX_t"]
#[derive(Clone, Debug)]
pub enum XMTRX_t {
    Var0(XMTRX_tVar0),
}
impl XMTRX_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = XMTRX_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:587:1"]
#[derive(Clone, Debug)]
pub struct N_0t_at_negVar0 {
    N_0: u8,
}
impl N_0t_at_negVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("@-"),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_at_neg"]
#[derive(Clone, Debug)]
pub enum N_0t_at_neg {
    Var0(N_0t_at_negVar0),
}
impl N_0t_at_neg {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_at_negVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:483:1"]
#[derive(Clone, Debug)]
pub struct ssr_tVar0 {}
impl ssr_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::SSR)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table ssr_t"]
#[derive(Clone, Debug)]
pub enum ssr_t {
    Var0(ssr_tVar0),
}
impl ssr_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = ssr_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:469:1"]
#[derive(Clone, Debug)]
pub struct ssr_N_0tVar0 {
    N_0: u8,
}
impl ssr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::SSR),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table ssr_N_0t"]
#[derive(Clone, Debug)]
pub enum ssr_N_0t {
    Var0(ssr_N_0tVar0),
}
impl ssr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = ssr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:461:1"]
#[derive(Clone, Debug)]
pub struct I_1tbranchVar0 {
    dest: i64,
}
impl I_1tbranchVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { dest } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *dest)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let I_1 = token_parser.I_1();
            *context = context_current;
            Some(((), (I_1), block_len))
        };
        let ((), (I_1), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut dest = 0i64;
        dest = (((i64::try_from(inst_start).unwrap()
            + i64::try_from(I_1).unwrap())
            * (2u64 as i64))
            + (4u64 as i64));
        let dest = 0i64;
        *context = context_current;
        Some((inst_len, Self { dest }))
    }
}
#[doc = "Table I_1tbranch"]
#[derive(Clone, Debug)]
pub enum I_1tbranch {
    Var0(I_1tbranchVar0),
}
impl I_1tbranch {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = I_1tbranchVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:477:1"]
#[derive(Clone, Debug)]
pub struct sr_tVar0 {}
impl sr_tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self {} = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Register(Register::SR)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let OP_0 = token_parser.OP_0();
            *context = context_current;
            Some(((), (OP_0), block_len))
        };
        let ((), (OP_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self {}))
    }
}
#[doc = "Table sr_t"]
#[derive(Clone, Debug)]
pub enum sr_t {
    Var0(sr_tVar0),
}
impl sr_t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = sr_tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:459:1"]
#[derive(Clone, Debug)]
pub struct I_0tbranchVar0 {
    dest: i64,
}
impl I_0tbranchVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { dest } = self;
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Signed(true, *dest)];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let I_0 = token_parser.I_0();
            *context = context_current;
            Some(((), (I_0), block_len))
        };
        let ((), (I_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut dest = 0i64;
        dest = (((i64::try_from(inst_start).unwrap()
            + i64::try_from(I_0).unwrap())
            * (2u64 as i64))
            + (4u64 as i64));
        let dest = 0i64;
        *context = context_current;
        Some((inst_len, Self { dest }))
    }
}
#[doc = "Table I_0tbranch"]
#[derive(Clone, Debug)]
pub enum I_0tbranch {
    Var0(I_0tbranchVar0),
}
impl I_0tbranch {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = I_0tbranchVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:533:1"]
#[derive(Clone, Debug)]
pub struct mach_N_0tVar0 {
    N_0: u8,
}
impl mach_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::MACH),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table mach_N_0t"]
#[derive(Clone, Debug)]
pub enum mach_N_0t {
    Var0(mach_N_0tVar0),
}
impl mach_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = mach_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:450:1"]
#[derive(Clone, Debug)]
pub struct I_0tVar0 {
    I_0: i8,
}
impl I_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { I_0 } = self;
        let extend: [DisplayElement; 2usize] = [
            DisplayElement::Literal("#"),
            DisplayElement::Signed(true, i64::try_from(*I_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let I_0 = token_parser.I_0();
            *context = context_current;
            Some(((), (I_0), block_len))
        };
        let ((), (I_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { I_0 }))
    }
}
#[doc = "Table I_0t"]
#[derive(Clone, Debug)]
pub enum I_0t {
    Var0(I_0tVar0),
}
impl I_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = I_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:577:1"]
#[derive(Clone, Debug)]
pub struct DRN_1tVar0 {
    DRN_1: u8,
}
impl DRN_1tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { DRN_1 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590061630544(usize::try_from(*DRN_1).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let DRN_1 = token_parser.DRN_1();
            *context = context_current;
            Some(((), (DRN_1), block_len))
        };
        let ((), (DRN_1), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { DRN_1 }))
    }
}
#[doc = "Table DRN_1t"]
#[derive(Clone, Debug)]
pub enum DRN_1t {
    Var0(DRN_1tVar0),
}
impl DRN_1t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = DRN_1tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:591:1"]
#[derive(Clone, Debug)]
pub struct U_2t_M0_dispr04Var0 {
    disp: i64,
    M_0: u8,
}
impl U_2t_M0_dispr04Var0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { disp, M_0 } = self;
        let extend: [DisplayElement; 5usize] = [
            DisplayElement::Literal("@("),
            DisplayElement::Signed(true, *disp),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*M_0).unwrap()),
            DisplayElement::Literal(")"),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let U_2 = token_parser.U_2();
            let M_0 = token_parser.M_0();
            *context = context_current;
            Some(((), (U_2, M_0), block_len))
        };
        let ((), (U_2, M_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        let mut disp = 0i64;
        disp = (i64::try_from(U_2).unwrap() * (4u64 as i64));
        let disp = 0i64;
        *context = context_current;
        Some((inst_len, Self { disp, M_0 }))
    }
}
#[doc = "Table U_2t_M0_dispr04"]
#[derive(Clone, Debug)]
pub enum U_2t_M0_dispr04 {
    Var0(U_2t_M0_dispr04Var0),
}
impl U_2t_M0_dispr04 {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = U_2t_M0_dispr04Var0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:502:1"]
#[derive(Clone, Debug)]
pub struct N_0t_dbrVar0 {
    N_0: u8,
}
impl N_0t_dbrVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
            DisplayElement::Literal(","),
            DisplayElement::Register(Register::DBR),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table N_0t_dbr"]
#[derive(Clone, Debug)]
pub enum N_0t_dbr {
    Var0(N_0t_dbrVar0),
}
impl N_0t_dbr {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = N_0t_dbrVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:473:1"]
#[derive(Clone, Debug)]
pub struct sgr_N_0tVar0 {
    N_0: u8,
}
impl sgr_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::SGR),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table sgr_N_0t"]
#[derive(Clone, Debug)]
pub enum sgr_N_0t {
    Var0(sgr_N_0tVar0),
}
impl sgr_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = sgr_N_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:444:1"]
#[derive(Clone, Debug)]
pub struct M_0tVar0 {
    M_0: u8,
}
impl M_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { M_0 } = self;
        let extend: [DisplayElement; 1usize] =
            [meaning_94590151099744(usize::try_from(*M_0).unwrap())];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let M_0 = token_parser.M_0();
            *context = context_current;
            Some(((), (M_0), block_len))
        };
        let ((), (M_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { M_0 }))
    }
}
#[doc = "Table M_0t"]
#[derive(Clone, Debug)]
pub enum M_0t {
    Var0(M_0tVar0),
}
impl M_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = M_0tVar0::parse(
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/SuperH4/data/languages/SuperH4.sinc:471:1"]
#[derive(Clone, Debug)]
pub struct spc_N_0tVar0 {
    N_0: u8,
}
impl spc_N_0tVar0 {
    pub fn display_extend<T>(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &T,
    ) where
        T: ContextTrait + Clone,
    {
        let Self { N_0 } = self;
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Register(Register::SPC),
            DisplayElement::Literal(","),
            meaning_94590151099744(usize::try_from(*N_0).unwrap()),
        ];
        display.extend_from_slice(&extend);
    }
    fn disassembly<'a, T>(
        &mut self,
        context_param: &mut T,
        inst_start: u32,
        inst_next: u32,
        global_set: &mut impl GlobalSetTrait,
    ) where
        T: ContextTrait + Clone,
    {
    }
    pub fn parse<'a, T>(
        tokens: &'a [u8],
        context: &mut T,
        inst_start: u32,
        global_set: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut inst_len = 0 as u32;
        let mut context_current = context.clone();
        let mut token_current = tokens;
        let mut block_0 = |tokens, context: &mut T| {
            let mut block_len = 0 as u32;
            let mut context_current = context.clone();
            let token_parser = TokenParser16::new(tokens)?;
            block_len = 2u64 as u32;
            let N_0 = token_parser.N_0();
            *context = context_current;
            Some(((), (N_0), block_len))
        };
        let ((), (N_0), block_len) =
            block_0(token_current, &mut context_current)?;
        token_current = &token_current[usize::try_from(block_len).unwrap()..];
        inst_len += block_len;
        *context = context_current;
        Some((inst_len, Self { N_0 }))
    }
}
#[doc = "Table spc_N_0t"]
#[derive(Clone, Debug)]
pub enum spc_N_0t {
    Var0(spc_N_0tVar0),
}
impl spc_N_0t {
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
        inst_start: u32,
        inst_next: u32,
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
        inst_start: u32,
        global_set_param: &mut impl GlobalSetTrait,
    ) -> Option<(u32, Self)>
    where
        T: ContextTrait + Clone,
    {
        let mut context_current = context_param.clone();
        if let Some((inst_next, parsed)) = spc_N_0tVar0::parse(
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
