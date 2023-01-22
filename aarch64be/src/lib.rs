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

#[no_mangle]
pub fn parse_default(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = SpacesStruct::default();
    context.register_mut().write_ImmS_ImmR_TestSet_raw(0).unwrap();
    context.register_mut().write_ImmS_LT_ImmR_raw(0).unwrap();
    context.register_mut().write_ImmS_EQ_ImmR_raw(0).unwrap();
    context.register_mut().write_ImmS_LT_ImmR_minus_1_raw(0).unwrap();
    context.register_mut().write_ImmS_ne_1f_raw(0).unwrap();
    context.register_mut().write_ImmS_ne_3f_raw(0).unwrap();
    context.register_mut().write_ShowMemTag_raw(0).unwrap();
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
