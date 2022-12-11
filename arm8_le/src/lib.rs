#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
pub mod disassembler;

#[cfg(test)]
mod test {
    use crate::disassembler::*;

    pub struct GlobalSetDummy {}
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    impl GlobalSetTrait for GlobalSetDummy {
        fn set_reg2Num(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_regNum(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_base(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_regInc(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_ISA_MODE(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_REToverride(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_T(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_counter(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_mask(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_shft(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_true(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_itmode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_TEEMode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_ARMcond(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_CALLoverride(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_LowBitCodeMode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_TMode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_ARMcondCk(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_counter2(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_LRset(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_condit(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_full(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }
    }

    #[test]
    fn disassembler_simple1() {
        let tokens: &[(u32, &str, &[u8])] = &[
            (0x10000, "b 0x1000c", &[0x01, 0x00, 0x00, 0xea]),
            (0x10004, "b 0x1000c", &[0x00, 0x00, 0x00, 0xea]),
            (0x10008, "b 0x1000c", &[0xff, 0xff, 0xff, 0xea]),
            (0x1000c, "cpy r0,r0", &[0x00, 0x00, 0xa0, 0xe1]),
        ];
        let context = SpacesStruct {
            register: ContextregisterStruct { chunk_0x0: [0; 8] },
        };
        let mut global_set = GlobalSetDummy {};
        for &(addr, output, token) in tokens.iter() {
            let mut context = context.clone();
            let parsed =
                parse_instruction(token, &mut context, addr, &mut global_set);
            match parsed {
                None => panic!("Instruction invalid"),
                Some((_inst_next, instruction)) => {
                    let display: String = instruction
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect();
                    assert_eq!(&display, output);
                }
            }
        }
    }

    #[test]
    fn disassembler_capstone() {
        //partially stolen from https://github.com/capstone-engine/capstone/blob/master/tests/test_arm.c
        //Original credits:
        /* Capstone Disassembler Engine */
        /* By Nguyen Anh Quynh <aquynh@gmail.com>, 2013 */

        let arm_asm: &[(u32, &str)] = &[
            (0xe5_2d_e0_04, "str lr,[sp,#-0x4]!"),
            (0xe5_22_83_e0, "str r8,[r2,#-0x3e0]!"),
            (0x0e_03_02_f1, "mcreq p2,0x0,r0,cr3,cr1,0x7"),
            (0xe3_a0_00_00, "mov r0,#0x0"),
            (0xe7_c1_30_02, "strb r3,[r1,r2]"),
            (0xe3_53_00_00, "cmp r3,#0x0"),
            (0xe2_a1_00_02, "adc r0,r1,#0x2"),
            (0xe0_a1_00_02, "adc r0,r1,r2"),
            (0xe0_a0_01_21, "adc r0,r0,r1, lsr #0x2"),
            (0xe0_b0_01_21, "adcs r0,r0,r1, lsr #0x2"),
            (0xe0_a1_03_32, "adc r0,r1,r2, lsr r3"),
            (0xe0_a1_01_22, "adc r0,r1,r2, lsr #0x2"),
            (0x50_4f_61_65, "subpl r6,pc,r5, ror #0x2"),
            (0xe5_53_30_30, "ldrb r3,[r3,#-0x30]"),
            (0xe1_df_10_b6, "ldrh r1,[0xe]"),
            (0xef_9f_00_02, "swi 0x9f0002"),
            //(0xea_27_c0_00, "b 0x9f0002"),
            (0xe1_a0_13_12, "mov r1,r2, lsl r3"),
            (0xe1_a0_11_82, "mov r1,r2, lsl #0x3"),
            (0xe1_a0_c0_00, "cpy r12,r0"),
            (0xe3_12_00_02, "tst r2,#0x2"),
            (0xe1_a0_12_51, "mov r1,r1, asr r2"),
            (0xe6_ef_10_72, "uxtb    r1,r2"),
            (0xee_b7_0a_e0, "vcvt.f64.f32\td0,s1"),
            (0xe1_91_0f_9f, "ldrex r0,[r1]"),
            (0xf4_20_06_0f, "vld1.8 {d0,d1,d2},[r0]"),
            (0xe6_a1_00_72, "sxtab   r0,r1,r2"),
            (0xf2_84_06_50, "vmov.i32 q0,simdExpand(0x0,0x6,0x40)"),
            (0xee_b8_e0_73, "mrc p0,0x5,lr,cr8,cr3,0x3"),
            (0xe6_81_02_12, "pkhbt r0,r1,r2, lsl #0x4"),
            (0xe6_a0_00_12, "ssat r0, #0x1, r2"),
            (0xe9_2d_60_03, "stmdb sp!,{r0,r1,sp,lr}"),
            (0xf4_60_40_8f, "vld4.32 {d20,d21,d22,d23},[r0]"),
            (0xe1_c2_00_d0, "ldrd r0,r1,[r2,#0x0]"),
            (0xf5_d0_f0_08, "pld [r0,#0x8]"),
            (0xec_bc_8b_10, "vldmia r12!,{d8,d9,d10,d11,d12,d13,d14,d15}"),
            (0xe1_d2_30_d4, "ldrsb r3,[r2,#0x4]"),
            (0xf2_be_0f_11, "vcvt.s32.f32\td0,d1,#0x2"),
            (0xe1_70_01_01, "cmn r0,r1, lsl #0x2"),
            (0xe2_91_00_06, "adds r0,r1,#0x6"),
            (0xf5_7f_f0_5b, "dmb ISH"),
            (0xe8_bd_20_00, "ldmia sp!,{sp}"),
            (0xe8_bd_a0_00, "ldmia sp!,{sp,pc}"),
            (0x00_0e_04_90, "muleq lr,r0,r4"),
            (0xe1_5f_10_b6, "ldrh r1,[0x2]"),
            (0xf1_01_02_00, "setend BE"),
            (0x00_00_80_f4, "strdeq r8,r9,[r0],-r4"),
        ];
        let context = SpacesStruct {
            register: ContextregisterStruct { chunk_0x0: [0; 8] },
        };
        let mut global_set = GlobalSetDummy {};
        let mut invalid = false;
        let mut invalid_parser = |tokens: &[u8], result, correct| {
            //speed-crunch helper
            let crunch = tokens
                .iter()
                .enumerate()
                //.map(|(i, t)| ((tokens.len() - 1 - i) * 8, t))
                .map(|(i, t)| format!("(0x{:02x}<<{})", t, i * 8))
                .reduce(|x, y| format!("{}+{}", x, y))
                .unwrap();
            println!("Crunch helper:  {}", crunch);
            let tokens = tokens.iter().fold(String::new(), |mut acc, x| {
                use core::fmt::Write;
                write!(acc, "\\x{:02x}", x).unwrap();
                acc
            });
            if let Some(result) = result {
                println!("Invalid parsed: {}", tokens);
                println!("Wrong result:   {}", result);
                println!("Correct result: {}", correct);
            } else {
                println!("Unable to parse {}", tokens);
                println!("Correct result: {}", correct);
            }
            println!();
            invalid = true;
        };
        for &(token, result) in arm_asm.iter() {
            let token = token.to_le_bytes();
            let token = &token;
            let mut context = context.clone();
            let parsed =
                parse_instruction(token, &mut context, 0, &mut global_set);
            match parsed {
                None => invalid_parser(token, None, result),
                Some((_inst_next, instruction)) => {
                    let display: String = instruction
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect();
                    if display != result {
                        invalid_parser(token, Some(display), result)
                    }
                }
            }
        }
        let thumb2_asm: &[(u32, &str)] = &[
            (0x00_00_47_70, "bx\tlr"),
            (0x00_00_dd_07, "ble\t0x12"),
            (0x00_00_47_00, "bx\tr0"),
            (0x00_00_47_01, "bx\tr0"),
            (0x00_00_47_02, "bx\tr0"),
            (0x00_00_bf_0a, "itet eq"),
            (0x00_00_24_f0, "movs r4,#0xf0"),
        ];
        let mut context = context.clone();
        context.register_mut().write_TMode_execution(1);
        let mut global_set = GlobalSetDummy {};
        for &(token, result) in thumb2_asm.iter() {
            let token = token.to_le_bytes();
            let token = &token;
            let mut context = context.clone();
            let parsed =
                parse_instruction(token, &mut context, 0, &mut global_set);
            match parsed {
                None => invalid_parser(token, None, result),
                Some((_inst_next, instruction)) => {
                    let display: String = instruction
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect();
                    if display != result {
                        invalid_parser(token, Some(display), result)
                    }
                }
            }
        }
        if invalid {
            panic!();
        }
    }
}
