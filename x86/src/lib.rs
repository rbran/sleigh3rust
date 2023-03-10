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
    parse!("Processors/x86/data/languages/x86.slaspec");
}
use crate::disassembler::*;

use std::fmt::Write;
fn parse(
    context: &mut SpacesStruct,
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let (addr, parsed) = parse_instruction(
        tokens,
        context,
        inst_start,
        &mut GlobalSetDefault::<SpacesStruct>::default(),
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
    let mut context = SpacesStruct::default();
    context.register_mut().write_addrsize_raw(0).unwrap();

    context.register_mut().write_bit64_raw(0).unwrap();
    context.register_mut().write_opsize_raw(0).unwrap();
    context.register_mut().write_segover_raw(0).unwrap();
    context.register_mut().write_protectedMode_raw(0).unwrap();

    context.register_mut().write_mandover_raw(0).unwrap();

    context.register_mut().write_rexWprefix_raw(0).unwrap();
    context.register_mut().write_rexRprefix_raw(0).unwrap();
    context.register_mut().write_rexXprefix_raw(0).unwrap();
    context.register_mut().write_rexBprefix_raw(0).unwrap();
    context.register_mut().write_rexprefix_raw(0).unwrap();

    context.register_mut().write_vexMode_raw(0).unwrap();
    context.register_mut().write_vexL_raw(0).unwrap();
    context.register_mut().write_vexVVVV_raw(0).unwrap();
    context.register_mut().write_vexMMMMM_raw(0).unwrap();

    context.register_mut().write_instrPhase_raw(0).unwrap();
    parse(&mut context, tokens, inst_start)
}

#[no_mangle]
pub fn parse_32bits(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = SpacesStruct::default();
    context.register_mut().write_addrsize_raw(1).unwrap();

    context.register_mut().write_bit64_raw(0).unwrap();
    context.register_mut().write_opsize_raw(1).unwrap();
    context.register_mut().write_segover_raw(0).unwrap();
    context.register_mut().write_protectedMode_raw(0).unwrap();

    context.register_mut().write_mandover_raw(0).unwrap();

    context.register_mut().write_rexWprefix_raw(0).unwrap();
    context.register_mut().write_rexRprefix_raw(0).unwrap();
    context.register_mut().write_rexXprefix_raw(0).unwrap();
    context.register_mut().write_rexBprefix_raw(0).unwrap();
    context.register_mut().write_rexprefix_raw(0).unwrap();

    context.register_mut().write_vexMode_raw(0).unwrap();
    context.register_mut().write_vexL_raw(0).unwrap();
    context.register_mut().write_vexVVVV_raw(0).unwrap();
    context.register_mut().write_vexMMMMM_raw(0).unwrap();

    context.register_mut().write_instrPhase_raw(0).unwrap();
    parse(&mut context, tokens, inst_start)
}
