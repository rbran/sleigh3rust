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
    fn set_PAIR_INSTRUCTION_FLAG(&mut self, address: Option<u64>, value: i64) {}
    fn set_REL6(&mut self, address: Option<u64>, value: i64) {}
    fn set_RELP(&mut self, address: Option<u64>, value: i64) {}
    fn set_ISA_MODE(&mut self, address: Option<u64>, value: i64) {}
    fn set_LowBitCodeMode(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_isjal(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_select(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_1005(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_1004(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_sa40(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_xreg(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_frame(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_areg(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_b0(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_b1(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_b2(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_b3(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_saz(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_1511(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_1511s(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_1411(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_value_1411s(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_tgt_2521(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_tgt_2016(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_tgt_x(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_is_ext(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_m16r32(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_m16r32a(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_reg_high(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_reg_low(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_svrs_sreg(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_svrs_xs(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_svrs_s1(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_svrs_s0(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_done(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_delay(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_t4_name(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_t4(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_tra(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_code(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_codes(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_addim(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_addims(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_imm2(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_imm2s(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_imm3(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_imm3s(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_imm5(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_imm5s(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_imm6(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_rlist(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_base(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_basea(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_rd(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_rdset(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_rs1(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_rs1lo(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_32_rs1set(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_16_rs(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_16_rslo(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_16_rshi(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_off16_s(&mut self, address: Option<u64>, value: i64) {}
    fn set_ext_off16_u(&mut self, address: Option<u64>, value: i64) {}
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
