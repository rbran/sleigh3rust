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
    fn set_longMode(&mut self, address: Option<u64>, value: i64) {}
    fn set_reserved(&mut self, address: Option<u64>, value: i64) {}
    fn set_addrsize(&mut self, address: Option<u64>, value: i64) {}
    fn set_bit64(&mut self, address: Option<u64>, value: i64) {}
    fn set_opsize(&mut self, address: Option<u64>, value: i64) {}
    fn set_segover(&mut self, address: Option<u64>, value: i64) {}
    fn set_highseg(&mut self, address: Option<u64>, value: i64) {}
    fn set_protectedMode(&mut self, address: Option<u64>, value: i64) {}
    fn set_repneprefx(&mut self, address: Option<u64>, value: i64) {}
    fn set_repprefx(&mut self, address: Option<u64>, value: i64) {}
    fn set_prefix_66(&mut self, address: Option<u64>, value: i64) {}
    fn set_prefix_f3(&mut self, address: Option<u64>, value: i64) {}
    fn set_prefix_f2(&mut self, address: Option<u64>, value: i64) {}
    fn set_mandover(&mut self, address: Option<u64>, value: i64) {}
    fn set_rexWprefix(&mut self, address: Option<u64>, value: i64) {}
    fn set_rexRprefix(&mut self, address: Option<u64>, value: i64) {}
    fn set_rexXprefix(&mut self, address: Option<u64>, value: i64) {}
    fn set_rexBprefix(&mut self, address: Option<u64>, value: i64) {}
    fn set_rexprefix(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexMode(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexL(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexVVVV(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexVVVV_r32(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexVVVV_r64(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexVVVV_XmmReg(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexVVVV_YmmReg(&mut self, address: Option<u64>, value: i64) {}
    fn set_vexMMMMM(&mut self, address: Option<u64>, value: i64) {}
    fn set_suffix3D(&mut self, address: Option<u64>, value: i64) {}
    fn set_instrPhase(&mut self, address: Option<u64>, value: i64) {}
}
fn parse(
    context: &mut SpacesStruct,
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let (addr, parsed) = parse_instruction(
        tokens,
        context,
        inst_start,
        &mut GlobalSetDummy,
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
    parse(&mut context, tokens, inst_start)
}

#[no_mangle]
pub fn parse_32bits(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = SpacesStruct::default();
    context.register_mut().write_addrsize_raw(1);
    context.register_mut().write_opsize_raw(1);
    parse(&mut context, tokens, inst_start)
}

#[no_mangle]
pub fn parse_64bits_emu32(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = SpacesStruct::default();
    context.register_mut().write_addrsize_raw(2);
    context.register_mut().write_bit64_raw(1);
    context.register_mut().write_opsize_raw(1);
    parse(&mut context, tokens, inst_start)
}

#[no_mangle]
pub fn parse_64bits(
    tokens: &[u8],
    inst_start: AddrType,
) -> Option<(AddrType, String)> {
    let mut context = SpacesStruct::default();
    context.register_mut().write_longMode_raw(1);
    context.register_mut().write_addrsize_raw(1);
    context.register_mut().write_bit64_raw(1);
    context.register_mut().write_opsize_raw(1);
    parse(&mut context, tokens, inst_start)
}
