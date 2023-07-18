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
    generate_disasembler!("../Processors/AARCH64/data/languages/AARCH64_AppleSilicon.slaspec");
}
use crate::disassembler::*;

use std::fmt::Write;

#[no_mangle]
pub fn parse_default(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = ContextMemory::default();
    context.write_ImmS_ImmR_TestSet(0);
    context.write_ImmS_LT_ImmR(0);
    context.write_ImmS_EQ_ImmR(0);
    context.write_ImmS_LT_ImmR_minus_1(0);
    context.write_ImmS_ne_1f(0);
    context.write_ImmS_ne_3f(0);
    context.write_ShowMemTag(0);
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
