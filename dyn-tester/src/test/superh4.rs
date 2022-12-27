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
const SUPERH4: &[(u32, Instruction, &str)] = &[
    (0x0001_0000, Instruction(0xd001), "mov.l 0x10008,r0"),
    (0x0001_0002, Instruction(0xd101), "mov.l 0x10008,r1"),
    (0x0001_0004, Instruction(0xd200), "mov.l 0x10008,r2"),
    (0x0001_0006, Instruction(0xd300), "mov.l 0x10008,r3"),
    (0x0001_0008, Instruction(0x400b), "jsr @r0"),
];

const LIBS: &[(Endian, &'static str)] =
    &[(Endian::Big, "superh4_be"), (Endian::Little, "superh4_le")];

#[test]
fn superh4() {
    for (endian, lib) in LIBS.iter() {
        let ld_lib = unsafe {
            libloading::Library::new(format!("../target/debug/lib{}.so", &lib))
                .unwrap()
        };
        let parse_fun: libloading::Symbol<
            fn(&'_ [u8], u32) -> Option<(u32, String)>,
        > = unsafe { ld_lib.get(b"parse_default\0").unwrap() };
        for (addr, instruction, output) in SUPERH4 {
            let token = instruction.to_tokens(*endian);
            let (_next_addr, parsed_output) =
                parse_fun(&token, *addr).expect(&output);
            assert_eq!(&remove_spaces(&parsed_output), output);
        }
    }
}
