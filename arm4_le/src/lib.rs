#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
mod disassembler {
    use sleigh2macro::generate_disasembler;
    generate_disasembler!("../Processors/ARM/data/languages/ARM4_le.slaspec");
}
use crate::disassembler::*;

use std::fmt::Write;

#[no_mangle]
pub fn parse_arm(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = ContextMemory::default();
    context.write_LRset(0);
    context.write_ARMcondCk(0);
    context.write_CALLoverride(0);
    context.write_REToverride(0);
    let mut globalset = GlobalSet::new(context.clone());
    let (addr, parsed) = parse_instruction(
        tokens,
        &mut context,
        inst_start,
        &mut globalset,
    )?;
    let mut output = String::new();
    for ele in parsed.into_iter() {
        write!(&mut output, "{}", ele).unwrap();
    }
    Some((addr, output))
}
