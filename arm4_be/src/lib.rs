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
    fn set_reg2Num(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_regNum(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_regInc(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_REToverride(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_counter(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_ARMcond(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_CALLoverride(&mut self, _inst_start: Option<AddrType>, _value: i64) {
    }
    fn set_ARMcondCk(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_counter2(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_LRset(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
}

#[no_mangle]
pub fn parse_default(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = SpacesStruct::default();
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
