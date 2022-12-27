use crate::test::remove_spaces;

use super::Endian;

struct Instruction(u16);
impl Instruction {
    fn to_tokens(&self, endian: Endian) -> [u8; 2] {
        match endian {
            Endian::Big => self.0.to_be_bytes(),
            Endian::Little => self.0.to_le_bytes(),
        }
    }
}

const PIC: &[(u16, Instruction, &str)] = &[
    (0x0000, Instruction(0x0a03), "GOTO 0x3"),
    (0x0001, Instruction(0x0a03), "GOTO 0x3"),
    (0x0002, Instruction(0x0a03), "GOTO 0x3"),
    (0x0003, Instruction(0x0c0f), "MOVLW #0xf"),
];

const LIBS: &[(Endian, &'static str)] =
    &[(Endian::Little, "pic12c5xx"), (Endian::Little, "pic16c5x")];

#[test]
fn pic() {
    for (endian, lib) in LIBS.iter() {
        let ld_lib = unsafe {
            libloading::Library::new(format!("../target/debug/lib{}.so", &lib))
                .unwrap()
        };
        let parse_fun: libloading::Symbol<
            fn(&'_ [u8], u16) -> Option<(u16, String)>,
        > = unsafe { ld_lib.get(b"parse_default\0").unwrap() };
        for (addr, instruction, output) in PIC {
            let token = instruction.to_tokens(*endian);
            let (_next_addr, parsed_output) =
                parse_fun(&token, *addr).expect(&output);
            assert_eq!(&remove_spaces(&parsed_output), output);
        }
    }
}
