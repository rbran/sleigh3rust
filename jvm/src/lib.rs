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
#[allow(non_snake_case)]
#[allow(unused_variables)]
impl GlobalSetTrait for GlobalSetDummy {
    fn set_switch_low(&mut self, address: Option<u32>, value: i64) {}
    fn set_switch_high(&mut self, address: Option<u32>, value: i64) {}
    fn set_switch_num(&mut self, address: Option<u32>, value: i64) {}
    fn set_in_table_switch(&mut self, address: Option<u32>, value: i64) {}
    fn set_in_lookup_switch(&mut self, address: Option<u32>, value: i64) {}
    fn set_alignmentPad(&mut self, address: Option<u32>, value: i64) {}
    fn set_padVal(&mut self, address: Option<u32>, value: i64) {}
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
