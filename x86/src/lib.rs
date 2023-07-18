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
    generate_disasembler!("../Processors/x86/data/languages/x86.slaspec");
}
use crate::disassembler::*;

use std::fmt::Write;
fn parse(
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
pub fn parse_16bits(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = ContextMemory::default();
    context.write_addrsize(0);

    context.write_bit64(0);
    context.write_opsize(0);
    context.write_segover(0);
    context.write_protectedMode(0);

    context.write_mandover(0);

    context.write_rexWprefix(0);
    context.write_rexRprefix(0);
    context.write_rexXprefix(0);
    context.write_rexBprefix(0);
    context.write_rexprefix(0);

    context.write_vexMode(0);
    context.write_vexL(0);
    context.write_vexVVVV(0);
    context.write_vexMMMMM(0);

    context.write_instrPhase(0);
    parse(&mut context, tokens, inst_start)
}

#[no_mangle]
pub fn parse_32bits(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = ContextMemory(0);
    context.write_addrsize(1);

    context.write_bit64(0);
    context.write_opsize(1);
    context.write_segover(0);
    context.write_protectedMode(0);

    context.write_mandover(0);

    context.write_rexWprefix(0);
    context.write_rexRprefix(0);
    context.write_rexXprefix(0);
    context.write_rexBprefix(0);
    context.write_rexprefix(0);

    context.write_vexMode(0);
    context.write_vexL(0);
    context.write_vexVVVV(0);
    context.write_vexMMMMM(0);

    context.write_instrPhase(0);
    parse(&mut context, tokens, inst_start)
}
