use crate::test::remove_spaces;

struct Instruction(u32);
impl Instruction {
    fn to_tokens(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const INSTRUCTIONS: &[(u32, Instruction, &str)] = &[
    (0x00038, Instruction(0x090001e1), "sethi %hi(0x78400),g4"),
    (0x0003c, Instruction(0x1029e36f), "illtrap 0x29e36f"),
    (0x00048, Instruction(0x0b000900), "sethi %hi(0x240000),g5"),
    (0x0009c, Instruction(0x09000900), "sethi %hi(0x240000),g4"),
    (0x000a0, Instruction(0xe36f264f), "prefetch [i4+0x64f],0x11"),
    (0x000a4, Instruction(0xf66e0b00), "ldstub [i0+g0],i3"),
    (0x000d4, Instruction(0x0b410900), "fbug,pn %fcc0,0x424d4"),
    (0x4f1e8, Instruction(0x008b01e1), "bn 0x30f96c"),
    (0x4f1ec, Instruction(0x13170b00), "sethi %hi(0x5c2c0000),o1"),
    (0x4f1f0, Instruction(0x09000900), "sethi %hi(0x240000),g4"),
    (0x4f1f4, Instruction(0x80345000), "orn l1,g0,g0"),
    (0x4f1fc, Instruction(0x962f5369), "andn i5,o1,o3"),
    (0xc09ac, Instruction(0xcc300b40), "sth g6,[g0]"),
    (0xc09b0, Instruction(0xb365047e), "movleu %icc,fp,i1"),
    (0xc09b4, Instruction(0xe36f264f), "prefetch [i4+0x64f],0x11"),
    (0xc09b8, Instruction(0xf66ef66c), "ldstub [i3+-0x994],i3"),
    (0xc09bc, Instruction(0xf66bf66a), "ldstub [o7+-0x996],i3"),
    (0xc09c0, Instruction(0xf669f668), "ldstub [g7+-0x998],i3"),
    (0xc09c4, Instruction(0x0b000900), "sethi %hi(0x240000),g5"),
    (0xc09c8, Instruction(0x44ea0300), "call 0x13b415c8"),
    (0xc09cc, Instruction(0xa012fcff), "or o3,-0x301,l0"),
    (0xc09d0, Instruction(0x9cd3fbff), "umulcc o7,-0x401,sp"),
    (0xc09d4, Instruction(0x24010000), "illtrap 0x10000"),
    (0xc09d8, Instruction(0x1c37fbff), "illtrap 0x37fbff"),
    (0xc09dc, Instruction(0x862f0fc7), "andn i4,g7,g3"),
    (0xc09e0, Instruction(0x962fc62f), "andn i7,o7,o3"),
];

#[test]
fn instructions() {
    let lib = "sparcv9_64";
    let ld_lib = unsafe {
        libloading::Library::new(format!("../target/debug/lib{}.so", &lib))
            .unwrap()
    };
    let parse_fun: libloading::Symbol<
        fn(&'_ [u8], u32) -> Option<(u32, String)>,
    > = unsafe { ld_lib.get(b"parse_default\0").unwrap() };
    for (addr, instruction, output) in INSTRUCTIONS {
        let token = instruction.to_tokens();
        let (_next_addr, parsed_output) =
            parse_fun(&token, *addr).expect(&output);
        assert_eq!(&remove_spaces(&parsed_output), output);
    }
}

