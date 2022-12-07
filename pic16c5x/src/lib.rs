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
    impl GlobalSetTrait for GlobalSetDummy {}

    #[test]
    fn disassembler_simple1() {
        let tokens: &[(u16, &str, &[u8])] = &[
            (0x0000, "GOTO 0x3", &[0x03, 0x0a]),
            (0x0001, "GOTO 0x3", &[0x03, 0x0a]),
            (0x0002, "GOTO 0x3", &[0x03, 0x0a]),
            (0x0003, "MOVLW #0xf", &[0x0f, 0x0c]),
        ];
        let context = SpacesStruct {};
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
