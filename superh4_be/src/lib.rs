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
        let tokens: &[(u32, &str, &[u8])] = &[
            (0x0001_0000, "mov.l 0x10008,r0", &[0xd0, 0x01]),
            (0x0001_0002, "mov.l 0x10008,r1", &[0xd1, 0x01]),
            (0x0001_0004, "mov.l 0x10008,r2", &[0xd2, 0x00]),
            (0x0001_0006, "mov.l 0x10008,r3", &[0xd3, 0x00]),
            (0x0001_0008, "jsr @r0", &[0x40, 0x0b]),
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
