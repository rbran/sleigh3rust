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
    fn set_RVA(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVB(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVC(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVD(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVE(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVF(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVG(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVH(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVI(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVJ(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVK(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVL(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVM(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVO(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVP(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVQ(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVR(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVS(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVT(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVU(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVV(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVW(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVX(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVY(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVZ(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_MXL(&mut self, inst_start: Option<AddrType>, value: i64) {}
    fn set_RVN(&mut self, inst_start: Option<AddrType>, value: i64) {}
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
