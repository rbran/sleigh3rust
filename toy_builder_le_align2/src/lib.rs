#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
mod disassembler;
use crate::disassembler::*;

use std::fmt::Write;

struct GlobalSetDummy;
impl GlobalSetTrait for GlobalSetDummy {
    fn set_fctx(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_nfctx(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_phase(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
    fn set_counter(&mut self, _inst_start: Option<AddrType>, _value: i64) {}
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
