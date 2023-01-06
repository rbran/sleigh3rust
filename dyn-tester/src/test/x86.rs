use crate::test::remove_spaces;

#[cfg_attr(rustfmt, rustfmt_skip)]
const STRLEN_32: &[(u32, &[u8], &str)] = &[
    (0x9fbd0, &[0xf3, 0x0f, 0x1e, 0xfb], "ENDBR32"),
    (0x9fbd4, &[0xe8, 0x22, 0x38, 0x0d, 0x00], "CALL 0x1733fb"),
    (0x9fbd9, &[0x81, 0xc2, 0x5b, 0x02, 0x18, 0x00], "ADD EDX,0x18025b"),
    (0x9fbdf, &[0x8b, 0x8a, 0xd4, 0x00, 0x00, 0x00], "MOV ECX,dword ptr [EDX + 0xd4]"),
    (0x9fbe5, &[0x8d, 0x82, 0x5c, 0x66, 0xe9, 0xff], "LEA EAX,[EDX + 0xffe9665c]"),
    (0x9fbeb, &[0xf6, 0x41, 0x7b, 0x04], "TEST byte ptr [ECX + 0x7b],0x4"),
    (0x9fbef, &[0x74, 0x16], "JZ 0x9fc07"),
    (0x9fbf1, &[0x8d, 0x82,0x1c, 0x37, 0xe8, 0xff], "LEA EAX,[EDX + 0xffe8371c]"),
    (0x9fbf7, &[0xf6, 0x81,0x7c, 0x01, 0x00, 0x00, 0x04], "TEST byte ptr [ECX + 0x17c],0x4"),
    (0x9fbfe, &[0x8d, 0x92,0x0c, 0x2b, 0xf7, 0xff], "LEA EDX,[EDX + 0xfff72b0c]"),
    (0x9fc04, &[0x0f, 0x45,0xc2], "CMOVNZ EAX,EDX"),
    (0x9fc07, &[0xc3], "RET"),
];
#[cfg_attr(rustfmt, rustfmt_skip)]
const STRLEN_64: &[(u64, &[u8], &str)] = &[
    (0x9d080, &[0xf3, 0x0f, 0x1e, 0xfa], "ENDBR64"),
    (0x9d084, &[0x48, 0x8b, 0x05, 0xcd, 0xad, 0x13, 0x00], "MOV RAX,qword ptr [0x1d7e58]"),
    (0x9d08b, &[0x48, 0x8d, 0x15, 0x4e, 0xad, 0x00, 0x00], "LEA RDX,[0xa7de0]"),
    (0x9d092, &[0x8b, 0x88, 0xb8, 0x00, 0x00, 0x00], "MOV ECX,dword ptr [RAX + 0xb8]"),
    (0x9d098, &[0x89, 0xce], "MOV ESI,ECX"),
    (0x9d09a, &[0x81, 0xe6, 0x28, 0x01, 0x00, 0x00], "AND ESI,0x128"),
    (0x9d0a0, &[0x81, 0xfe, 0x28, 0x01, 0x00, 0x00], "CMP ESI,0x128"),
    (0x9d0a6, &[0x74, 0x08], "JZ 0x19d0b0"),
    (0x9d0a8, &[0x48, 0x89, 0xd0], "MOV RAX"),
    (0x9d0ab, &[0xc3], "RET"),
];

#[test]
fn x86() {
    let ld_lib = unsafe {
        libloading::Library::new("../target/debug/libx86.so").unwrap()
    };
    let parse_fun: libloading::Symbol<
        fn(&'_ [u8], u32) -> Option<(u32, String)>,
    > = unsafe { ld_lib.get(b"parse_32bits\0").unwrap() };
    for (addr, instruction, output) in STRLEN_32 {
        let (_next_addr, parsed_output) =
            parse_fun(instruction, *addr).expect(&output);
        assert_eq!(&remove_spaces(&parsed_output), output);
    }
}

#[test]
fn x86_64() {
    let ld_lib = unsafe {
        libloading::Library::new("../target/debug/libx86_64.so").unwrap()
    };
    let parse_32: libloading::Symbol<
        fn(&'_ [u8], u64) -> Option<(u64, String)>,
    > = unsafe { ld_lib.get(b"parse_32bits\0").unwrap() };
    let parse_64: libloading::Symbol<
        fn(&'_ [u8], u64) -> Option<(u64, String)>,
    > = unsafe { ld_lib.get(b"parse_64bits\0").unwrap() };
    for (addr, instruction, output) in STRLEN_32 {
        let (_next_addr, parsed_output) =
            parse_32(instruction, *addr as u64).expect(&output);
        assert_eq!(&remove_spaces(&parsed_output), output);
    }
    for (addr, instruction, output) in STRLEN_64 {
        let (_next_addr, parsed_output) =
            parse_64(instruction, *addr).expect(&output);
        assert_eq!(&remove_spaces(&parsed_output), output);
    }
}
