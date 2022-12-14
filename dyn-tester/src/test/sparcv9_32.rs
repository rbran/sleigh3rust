use crate::test::remove_spaces;

struct Instruction(u32);
impl Instruction {
    fn to_tokens(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const SPARC: &[(u32, Instruction, &str)] = &[
    (0x000, Instruction(0x81c3e008), "retl"),
    (0x004, Instruction(0xae03c017), "add o7,l7,l7"),
    (0x008, Instruction(0x9de3bf98), "save sp,-0x68,sp"),
    (0x00c, Instruction(0x113ffffc), "sethi %hi(0xfffff000),o0"),
    (0x010, Instruction(0x90122004), "or o0,0x4,o0"),
    (0x014, Instruction(0x2f000377), "sethi %hi(0xddc00),l7"),
    (0x018, Instruction(0x7ffffffa), "call 0x0"),
    (0x01c, Instruction(0xae05e154), "add l7,0x154,l7"),
    (0x020, Instruction(0xe005c008), "lduw [l7+o0],l0"),
    (0x024, Instruction(0x10800004), "ba 0x34"),
    (0x028, Instruction(0xa0042004), "add l0,0x4,l0"),
    (0x02c, Instruction(0x9fc20000), "jmpl [o0+g0],o7"),
    (0x034, Instruction(0xd0040000), "lduw [l0+g0],o0"),
    (0x038, Instruction(0x80a22000), "cmp o0,0x0"),
    (0x03c, Instruction(0x12bffffc), "bne 0x2c"),
    (0x040, Instruction(0x01000000), "nop"),
    (0x044, Instruction(0x81c7e008), "ret"),
    (0x048, Instruction(0x81e80000), "restore"),
    (0x04c, Instruction(0x00000000), "illtrap 0x0"),
    (0x060, Instruction(0x9012200c), "or o0,0xc,o0"),
    (0x06c, Instruction(0xae05e104), "add l7,0x104,l7"),
    (0x070, Instruction(0xd205c008), "lduw [l7+o0],o1"),
    (0x074, Instruction(0xd0024000), "lduw [o1+g0],o0"),
    (0x07c, Instruction(0x12800010), "bne 0xbc"),
    (0x080, Instruction(0x133ffffc), "sethi %hi(0xfffff000),o1"),
    (0x08c, Instruction(0x90122014), "or o0,0x14,o0"),
    (0x090, Instruction(0x92126018), "or o1,0x18,o1"),
    (0x094, Instruction(0xd405c008), "lduw [l7+o0],o2"),
    (0x098, Instruction(0xd605c009), "lduw [l7+o1],o3"),
    (0x09c, Instruction(0xd4028000), "lduw [o2+g0],o2"),
    (0x0a0, Instruction(0xd002c000), "lduw [o3+g0],o0"),
    (0x0a4, Instruction(0x80a28008), "cmp o2,o0"),
    (0x0a8, Instruction(0x02800005), "be 0xbc"),
    (0x0b0, Instruction(0x40001cce), "call 0x73e8"),
    (0x0b4, Instruction(0x9010000a), "mov o2,o0"),
    (0x0bc, Instruction(0x92126010), "or o1,0x10,o1"),
    (0x0c0, Instruction(0xd805c009), "lduw [l7+o1],o4"),
    (0x0c4, Instruction(0x153ffffc), "sethi %hi(0xfffff000),o2"),
    (0x0c8, Instruction(0x9412a01c), "or o2,0x1c,o2"),
    (0x0d0, Instruction(0xd205c00a), "lduw [l7+o2],o1"),
    (0x0d4, Instruction(0x90122020), "or o0,0x20,o0"),
    (0x0d8, Instruction(0xd605c008), "lduw [l7+o0],o3"),
    (0x0dc, Instruction(0xf2230000), "stw i1,[o4+g0]"),
    (0x0e0, Instruction(0xf4224000), "stw i2,[o1+g0]"),
    (0x0e4, Instruction(0x90100018), "mov i0,o0"),
    (0x0e8, Instruction(0xf022c000), "stw i0,[o3+g0]"),
    (0x0ec, Instruction(0x92100019), "mov i1,o1"),
    (0x0f0, Instruction(0x400000a8), "call 0x390"),
    (0x0f4, Instruction(0x9410001a), "mov i2,o2"),
    (0x0f8, Instruction(0x4001ce2e), "call 0x739b0"),
    (0x0fc, Instruction(0x9010001a), "mov i2,o0"),
    (0x100, Instruction(0x7fffffc2), "call 0x8"),
    (0x110, Instruction(0x9de3bfc0), "save sp,-0x40,sp"),
    (0x114, Instruction(0x40000002), "call 0x11c"),
    (0x11c, Instruction(0xae15e058), "or l7,0x58,l7"),
    (0x120, Instruction(0xae05c00f), "add l7,o7,l7"),
    (0x124, Instruction(0x253ffffc), "sethi %hi(0xfffff000),l2"),
    (0x128, Instruction(0xa414a024), "or l2,0x24,l2"),
    (0x12c, Instruction(0xe405c012), "lduw [l7+l2],l2"),
    (0x130, Instruction(0x80a4a000), "cmp l2,0x0"),
    (0x138, Instruction(0x273ffffc), "sethi %hi(0xfffff000),l3"),
    (0x13c, Instruction(0xe4048000), "lduw [l2+g0],l2"),
    (0x140, Instruction(0x80a00012), "cmp g0,l2"),
    (0x144, Instruction(0xa4603fff), "subc g0,-0x1,l2"),
    (0x148, Instruction(0xa614e00c), "or l3,0xc,l3"),
    (0x14c, Instruction(0xe605c013), "lduw [l7+l3],l3"),
    (0x154, Instruction(0xe424c000), "stw l2,[l3+g0]"),
    (0x158, Instruction(0x12bfffc0), "bne 0x58"),
    (0x160, Instruction(0xd003a058), "lduw [sp+0x58],o0"),
    (0x164, Instruction(0x9203a05c), "add sp,0x5c,o1"),
    (0x168, Instruction(0x952a2002), "sll o0,0x2,o2"),
    (0x16c, Instruction(0x94028009), "add o2,o1,o2"),
    (0x170, Instruction(0x10bfffba), "ba 0x58"),
    (0x174, Instruction(0x9402a004), "add o2,0x4,o2"),
    (0x184, Instruction(0x40036f39), "call 0xdbe68"),
    (0x19c, Instruction(0x2f000376), "sethi %hi(0xdd800),l7"),
    (0x1a0, Instruction(0x7ffffffb), "call 0x18c"),
    (0x1a4, Instruction(0xae05e3cc), "add l7,0x3cc,l7"),
    (0x1a8, Instruction(0x90122028), "or o0,0x28,o0"),
    (0x1ac, Instruction(0xd805c008), "lduw [l7+o0],o4"),
    (0x1b8, Instruction(0x9212602c), "or o1,0x2c,o1"),
    (0x1bc, Instruction(0x9012201c), "or o0,0x1c,o0"),
    (0x1c4, Instruction(0x952e6002), "sll i1,0x2,o2"),
    (0x1d0, Instruction(0xd2030000), "lduw [o4+g0],o1"),
    (0x1d4, Instruction(0x9406800a), "add i2,o2,o2"),
    (0x1d8, Instruction(0xd007a05c), "lduw [fp+0x5c],o0"),
    (0x1dc, Instruction(0xd4240000), "stw o2,[l0+g0]"),
    (0x1e0, Instruction(0x80a26000), "cmp o1,0x0"),
    (0x1e4, Instruction(0x02800004), "be 0x1f4"),
    (0x1e8, Instruction(0xd022c000), "stw o0,[o3+g0]"),
    (0x1ec, Instruction(0x4000005b), "call 0x358"),
    (0x1f4, Instruction(0x80a76000), "cmp i5,0x0"),
    (0x200, Instruction(0x4003716f), "call 0xdc7bc"),
    (0x204, Instruction(0x9010001d), "mov i5,o0"),
    (0x20c, Instruction(0x90122030), "or o0,0x30,o0"),
    (0x210, Instruction(0xfa05c008), "lduw [l7+o0],i5"),
    (0x214, Instruction(0xd2074000), "lduw [i5+g0],o1"),
    (0x21c, Instruction(0x02800007), "be 0x238"),
    (0x224, Instruction(0x90122034), "or o0,0x34,o0"),
    (0x22c, Instruction(0x94102000), "mov 0x0,o2"),
    (0x230, Instruction(0x40036f68), "call 0xdbfd0"),
    (0x234, Instruction(0x90102001), "mov 0x1,o0"),
    (0x238, Instruction(0xd4040000), "lduw [l0+g0],o2"),
    (0x23c, Instruction(0x90100019), "mov i1,o0"),
    (0x240, Instruction(0x40037015), "call 0xdc294"),
    (0x244, Instruction(0x9210001a), "mov i2,o1"),
    (0x248, Instruction(0x80a72000), "cmp i4,0x0"),
    (0x24c, Instruction(0x22800005), "be,a 0x260"),
    (0x250, Instruction(0xd0074000), "lduw [i5+g0],o0"),
    (0x254, Instruction(0x4003715a), "call 0xdc7bc"),
    (0x258, Instruction(0x9010001c), "mov i4,o0"),
    (0x264, Instruction(0x0280000b), "be 0x290"),
    (0x268, Instruction(0x393ffffc), "sethi %hi(0xfffff000),i4"),
    (0x270, Instruction(0x9012203c), "or o0,0x3c,o0"),
    (0x274, Instruction(0x94172038), "or i4,0x38,o2"),
    (0x27c, Instruction(0x98102000), "mov 0x0,o4"),
    (0x280, Instruction(0xd605c00a), "lduw [l7+o2],o3"),
    (0x288, Instruction(0x40036f52), "call 0xdbfd0"),
    (0x28c, Instruction(0xd4068000), "lduw [i2+g0],o2"),
    (0x290, Instruction(0x80a6e000), "cmp i3,0x0"),
    (0x29c, Instruction(0x9fc6c000), "jmpl [i3+g0],o7"),
    (0x2ac, Instruction(0x0280000a), "be 0x2d4"),
    (0x2b4, Instruction(0x9412a040), "or o2,0x40,o2"),
    (0x2b8, Instruction(0x90172038), "or i4,0x38,o0"),
    (0x2cc, Instruction(0x40036f41), "call 0xdbfd0"),
    (0x2dc, Instruction(0x9fc60000), "jmpl [i0+g0],o7"),
    (0x2e4, Instruction(0x400371cf), "call 0xdca20"),
    (0x2f8, Instruction(0x7fffffa5), "call 0x18c"),
    (0x2fc, Instruction(0xae05e274), "add l7,0x274,l7"),
    (0x300, Instruction(0x40036d96), "call 0xdb958"),
    (0x304, Instruction(0x92102001), "mov 0x1,o1"),
    (0x308, Instruction(0x80a23fff), "cmp o0,-0x1"),
    (0x30c, Instruction(0x12800011), "bne 0x350"),
    (0x314, Instruction(0x400371bd), "call 0xdca08"),
];

#[test]
fn sparcv9_32() {
    let lib = "sparcv9_32";
    let ld_lib = unsafe {
        libloading::Library::new(format!("../target/debug/lib{}.so", &lib))
            .unwrap()
    };
    let parse_fun: libloading::Symbol<
        fn(&'_ [u8], u32) -> Option<(u32, String)>,
    > = unsafe { ld_lib.get(b"parse_default\0").unwrap() };
    for (addr, instruction, output) in SPARC {
        let token = instruction.to_tokens();
        let (_next_addr, parsed_output) =
            parse_fun(&token, *addr).expect(&output);
        assert_eq!(&remove_spaces(&parsed_output), output);
    }
}

