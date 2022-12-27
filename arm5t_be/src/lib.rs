#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
pub mod disassembler;
use crate::disassembler::*;

use std::fmt::Write;

pub struct GlobalSetDummy;
#[allow(non_snake_case)]
impl GlobalSetTrait for GlobalSetDummy {
    fn set_reg2Num(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_regNum(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_regInc(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_ISA_MODE(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_REToverride(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_T(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_counter(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_ARMcond(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_CALLoverride(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_LowBitCodeMode(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_TMode(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_ARMcondCk(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_counter2(&mut self, _address: Option<u32>, _value: i64) {}
    fn set_LRset(&mut self, _address: Option<u32>, _value: i64) {}
}

#[no_mangle]
pub fn parse_default(tokens: &[u8], inst_start: u32) -> Option<(u32, String)> {
    let mut context = SpacesStruct {
        register: ContextregisterStruct { chunk_0x0: [0; 8] },
    };
    let (addr, parsed) = parse_instruction(
        tokens,
        &mut context,
        inst_start,
        &mut GlobalSetDummy,
    )?;
    let mut output = String::new();
    for ele in parsed.into_iter() {
        write!(&mut output, "{}", ele).unwrap();
    }
    Some((addr, output))
}

#[no_mangle]
pub fn parse_thumb(tokens: &[u8], inst_start: u32) -> Option<(u32, String)> {
    let mut context = SpacesStruct {
        register: ContextregisterStruct { chunk_0x0: [0; 8] },
    };
    context.register_mut().write_TMode_raw(1);
    let (addr, parsed) = parse_instruction(
        tokens,
        &mut context,
        inst_start,
        &mut GlobalSetDummy,
    )?;
    let mut output = String::new();
    for ele in parsed.into_iter() {
        write!(&mut output, "{}", ele).unwrap();
    }
    Some((addr, output))
}
