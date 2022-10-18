pub trait GlobalSetTrait {
    fn set_test(&mut self, address: Option<u64>, value: u8);
}
pub trait ContextTrait {
    fn read_test(&self) -> u8;
    fn write_test(&mut self, value: u8);
}
#[derive(Debug, Clone, Copy)]
pub struct ContextSpaceregister {
    pub chunk_0x100: [u8; 4usize],
}
impl ContextSpaceregister {
    pub fn read(&self, addr: usize, buf: &mut [u8]) {
        let addr_end = addr + buf.len();
        match (addr, addr_end) {
            (256usize..=259usize, 256usize..=260usize) => {
                let start = addr - 256usize;
                let end = start + buf.len();
                buf.copy_from_slice(&self.chunk_0x100[start..end]);
            }
            _ => panic!("undefined mem {}:{}", addr, buf.len()),
        }
    }
    pub fn write(&mut self, addr: usize, buf: &[u8]) {
        let addr_end = addr + buf.len();
        match (addr, addr_end) {
            (256usize..=259usize, 256usize..=260usize) => {
                let start = addr - 256usize;
                let end = start + buf.len();
                self.chunk_0x100[start..end].copy_from_slice(buf);
            }
            _ => panic!("undefined mem {}:{}", addr, buf.len()),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub register: ContextSpaceregister,
}
impl ContextTrait for Context {
    fn read_test(&self) -> u8 {
        let mut mem = [0u8; 1usize];
        self.register.read(256usize, &mut mem[0usize..]);
        let mut work = u8::from_be_bytes(mem);
        work >>= 1usize;
        work &= 1usize as u8;
        work as u8
    }
    fn write_test(&mut self, value: u8) {
        let mut mem = [0u8; 1usize];
        self.register.read(256usize, &mut mem[0usize..]);
        let mut mem = u8::from_be_bytes(mem);
        mem &= !((1usize as u8) << 1usize);
        let mut param = value as u8;
        param &= 1usize as u8;
        param <<= 1usize;
        let result = (mem | param).to_be_bytes();
        self.register.write(256usize, &result[0usize..]);
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Register {
    test,
    contextreg,
    r0,
    sp,
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::test => write!(f, "test"),
            Self::contextreg => write!(f, "contextreg"),
            Self::r0 => write!(f, "r0"),
            Self::sp => write!(f, "sp"),
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
#[doc = "Constructor at /home/rbran/src/ghidra/Ghidra/Processors/DATA/data/languages/data.sinc:26:1"]
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
        let extend: [DisplayElement; 1usize] = [DisplayElement::Literal("nop")];
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
            if context.read_test() != (1u64 as i64) as u8 {
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
#[doc = "Table instruction"]
#[derive(Clone, Debug)]
pub enum instruction {
    Var0(instructionVar0),
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
        None
    }
}
