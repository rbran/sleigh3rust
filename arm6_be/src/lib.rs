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
    generate_disasembler!("../Processors/ARM/data/languages/ARM6_be.slaspec");
}
use crate::disassembler::*;

use std::fmt::Write;

#[no_mangle]
pub fn parse(
    context: &mut ContextMemory,
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut globalset = GlobalSet::new(context.clone());
    let (addr, parsed) = parse_instruction(
        tokens,
        context,
        inst_start,
        &mut globalset,
    )?;
    let mut output = String::new();
    for ele in parsed.into_iter() {
        write!(&mut output, "{}", ele).unwrap();
    }
    Some((addr, output))
}

#[no_mangle]
pub fn parse_thumb(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = ContextMemory::default();
    context.write_TMode(1);
    context.write_LRset(0);
    context.write_ARMcondCk(0);
    context.write_CALLoverride(0);
    context.write_REToverride(0);
    context.write_itmode(0);
    context.write_TEEMode(0);
    parse(&mut context, tokens, inst_start)
}

#[no_mangle]
pub fn parse_arm(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = ContextMemory(0);
    context.write_TMode(0);
    context.write_LRset(0);
    context.write_ARMcondCk(0);
    context.write_CALLoverride(0);
    context.write_REToverride(0);
    context.write_itmode(0);
    parse(&mut context, tokens, inst_start)
}
