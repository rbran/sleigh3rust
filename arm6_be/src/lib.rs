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
        fn set_CALLoverride(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_base(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_ARMcondCk(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_regInc(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_TEEMode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_REToverride(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_regNum(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_counter(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_ARMcond(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_LRset(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_shft(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_itmode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_condit(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_LowBitCodeMode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_full(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_TMode(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_counter2(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_T(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_mask(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_ISA_MODE(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_cond_true(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }

        fn set_reg2Num(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }
    }

    #[test]
    fn disassembler_simple1() {
        let tokens_be: &[(u32, &str, &[u8])] = &[
            (0x10000, "b 0x1000c", &[0xea, 0x00, 0x00, 0x01]),
            (0x10004, "b 0x1000c", &[0xea, 0x00, 0x00, 0x00]),
            (0x10008, "b 0x1000c", &[0xea, 0xff, 0xff, 0xff]),
            (0x1000c, "cpy r0,r0", &[0xe1, 0xa0, 0x00, 0x00]),
        ];
        let be_context = SpacesStruct {
            register: ContextregisterStruct { chunk_0x0: [0; 8] },
        };
        let mut global_set = GlobalSetDummy {};
        for &(addr, output, token) in tokens_be.iter() {
            let mut context = be_context.clone();
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
}
