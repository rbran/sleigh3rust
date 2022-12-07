
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
    }

    #[test]
    fn disassembler_simple1() {
        let tokens: &[(u32, &str, &[u8])] = &[
            //TODO: bin code
        ];
        let context = SpacesStruct {
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
