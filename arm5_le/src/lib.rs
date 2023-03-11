#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
mod disassembler {
    use sleigh2macro::parse;
    parse!("Processors/ARM/data/languages/ARM5_le.slaspec");
}
use crate::disassembler::*;

use std::fmt::Write;

#[no_mangle]
pub fn parse_arm(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = SpacesStruct::default();
    context.register_mut().write_LRset_raw(0).unwrap();
    context.register_mut().write_ARMcondCk_raw(0).unwrap();
    context.register_mut().write_CALLoverride_raw(0).unwrap();
    context.register_mut().write_REToverride_raw(0).unwrap();
    let (addr, parsed) = parse_instruction(
        tokens,
        &mut context,
        inst_start,
        &mut GlobalSetDefault::<SpacesStruct>::default(),
    )?;
    let mut output = String::new();
    for ele in parsed.into_iter() {
        write!(&mut output, "{}", ele).unwrap();
    }
    Some((addr, output))
}
