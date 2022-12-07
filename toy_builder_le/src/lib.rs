#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
mod disassembler;

#[cfg(test)]
mod tests {
    use crate::disassembler::*;

    pub struct GlobalSetDummy {}
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    impl GlobalSetTrait for GlobalSetDummy {
        fn set_nfctx(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }
        fn set_phase(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }
        fn set_counter(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }
        fn set_fctx(&mut self, address: Option<u32>, value: i64) {
            todo!()
        }
    }

    #[test]
    fn disassembler_simple1() {
        let tokens: &[(u32, &str, &[u8])] = &[
            //TODO: bin code
        ];
        let context = SpacesStruct {
            register: ContextregisterStruct {
                chunk_0x0: [0u8; 8],
            },
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
}
