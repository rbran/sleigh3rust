use crate::test::remove_spaces;

use super::Endian;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ArmVersion {
    V4,
    V5,
    V6,
    V7,
    V8,
}
use ArmVersion::*;
use Endian::*;

struct Lib {
    version: ArmVersion,
    thumb: bool,
    endian: Endian,
    lib: &'static str,
}
#[cfg_attr(rustfmt, rustfmt_skip)]
const LIBS: &[Lib] = &[
    Lib { version: V4, thumb: false, endian: Big   , lib: "arm4_be"},
    Lib { version: V4, thumb: false, endian: Little, lib: "arm4_le"},
    Lib { version: V4, thumb: true,  endian: Big   , lib: "arm4t_be"},
    Lib { version: V4, thumb: true,  endian: Little, lib: "arm4t_le"},
    Lib { version: V5, thumb: false, endian: Big   , lib: "arm5_be"},
    Lib { version: V5, thumb: false, endian: Little, lib: "arm5_le"},
    Lib { version: V5, thumb: true,  endian: Big   , lib: "arm5t_be"},
    Lib { version: V5, thumb: true,  endian: Little, lib: "arm5t_le"},
    Lib { version: V6, thumb: true,  endian: Big   , lib: "arm6_be"},
    Lib { version: V6, thumb: true,  endian: Little, lib: "arm6_le"},
    Lib { version: V7, thumb: true,  endian: Big   , lib: "arm7_be"},
    Lib { version: V7, thumb: true,  endian: Little, lib: "arm7_le"},
    Lib { version: V8, thumb: true,  endian: Big   , lib: "arm8_be"},
    Lib { version: V8, thumb: true,  endian: Little, lib: "arm8_le"},
];

enum Instruction {
    Arm(u32),
    Thumb32(u16, u16),
    Thumb16(u16),
}
impl Instruction {
    fn to_tokens(&self, endian: Endian) -> Vec<u8> {
        use Endian::*;
        match (self, endian) {
            (Arm(x), Big) => x.to_be_bytes().to_vec(),
            (Arm(x), Little) => x.to_le_bytes().to_vec(),
            (Thumb32(x, y), Big) => {
                ((*x as u32) << 16 | *y as u32).to_be_bytes().to_vec()
            }
            (Thumb32(x, y), Little) => {
                ((*y as u32) << 16 | *x as u32).to_le_bytes().to_vec()
            }
            (Thumb16(x), Big) => x.to_be_bytes().to_vec(),
            (Thumb16(x), Little) => x.to_le_bytes().to_vec(),
        }
    }
    fn thumb_mode(&self) -> bool {
        match self {
            Arm(_) => false,
            Thumb32(..) | Thumb16(_) => true,
        }
    }
}
use Instruction::*;

#[cfg_attr(rustfmt, rustfmt_skip)]
const ARM: &[(fn(ArmVersion) -> bool, &[(Instruction, &'static str)])] = &[
(|_| true, &[
    (Arm(0xe52d_e004), "str lr,[sp,#-0x4]!"),
    (Arm(0xe522_83e0), "str r8,[r2,#-0x3e0]!"),
    (Arm(0x0e03_02f1), "mcreq p2,0x0,r0,cr3,cr1,0x7"),
    (Arm(0xe3a0_0000), "mov r0,#0x0"),
    (Arm(0xe7c1_3002), "strb r3,[r1,r2]"),
    (Arm(0xe2a1_0002), "adc r0,r1,#0x2"),
    (Arm(0xe0a1_0002), "adc r0,r1,r2"),
    (Arm(0xe0a0_0121), "adc r0,r0,r1, lsr #0x2"),
    (Arm(0xe0b0_0121), "adcs r0,r0,r1, lsr #0x2"),
    (Arm(0xe0a1_0332), "adc r0,r1,r2, lsr r3"),
    (Arm(0xe0a1_0122), "adc r0,r1,r2, lsr #0x2"),
    (Arm(0x504f_6165), "subpl r6,pc,r5, ror #0x2"),
    (Arm(0xe553_3030), "ldrb r3,[r3,#-0x30]"),
    (Arm(0xe1df_10b6), "ldrh r1,[0xe]"),
    (Arm(0xef9f_0002), "swi 0x9f0002"),
    (Arm(0xe1a0_1312), "mov r1,r2, lsl r3"),
    (Arm(0xe1a0_1182), "mov r1,r2, lsl #0x3"),
    (Arm(0xe312_0002), "tst r2,#0x2"),
    (Arm(0xe1a0_1251), "mov r1,r1, asr r2"),
    (Arm(0xeeb8_e073), "mrc p0,0x5,lr,cr8,cr3,0x3"),
    (Arm(0xe92d_6003), "stmdb sp!,{r0,r1,sp,lr}"),
    (Arm(0xe1d2_30d4), "ldrsb r3,[r2,#0x4]"),
    (Arm(0xe8bd_2000), "ldmia sp!,{sp}"),
    (Arm(0xe8bd_a000), "ldmia sp!,{sp,pc}"),
    (Arm(0x000e_0490), "muleq lr,r0,r4"),
    (Arm(0xe15f_10b6), "ldrh r1,[0x2]"),
    (Arm(0xe170_0101), "cmn r0,r1, lsl #0x2"),
    (Arm(0xe353_0000), "cmp r3,#0x0"),
    (Arm(0xda00_0003), "ble 0x14"),
    (Arm(0xe3a0_40f0), "mov r4,#0xf0"),
    (Arm(0xe0a0_0001), "adc r0,r0,r1"),
    (Thumb16(0xdd08),  "ble 0x14"),
    (Thumb16(0x4700),  "bx r0"),
    (Thumb16(0x4708),  "bx r1"),
    (Thumb16(0x4710),  "bx r2"),
    (Thumb16(0x4770),  "bx lr"),
    (Thumb16(0x24f0),  "mov r4,#0xf0"),
]),
(|v| v >= V5, &[
    (Arm(0xda00_0003), "ble 0x14"),
    (Arm(0xe12f_ff10), "bx r0"),
    (Arm(0xe12f_ff11), "bx r1"),
    (Arm(0xe12f_ff12), "bx r2"),
    (Arm(0xe12f_ff1e), "bx lr"),
    (Arm(0xe3a0_40f0), "mov r4,#0xf0"),
]),
(|v| v <= V5, &[
    //Pre UAL
    (Arm(0xe1a0_c000), "mov r12,r0"),
]),
(|v| v >= V6, &[
    //Post UAL
    (Arm(0xe1a0_c000),        "cpy r12,r0"),
    (Arm(0xe6ef_1072),        "uxtb r1,r2"),
    (Arm(0xeeb7_0ae0),        "vcvt.f64.f32 d0,s1"),
    (Arm(0xe191_0f9f),        "ldrex r0,[r1]"),
    (Arm(0xe6a1_0072),        "sxtab r0,r1,r2"),
    (Arm(0xe681_0212),        "pkhbt r0,r1,r2, lsl #0x4"),
    (Arm(0xe6a0_0012),        "ssat r0, #0x1, r2"),
    (Arm(0xe1c2_00d0),        "ldrd r0,r1,[r2,#0x0]"),
    (Arm(0xf5d0_f008),        "pld [r0,#0x8]"),
    (Arm(0xecbc_8b10),        "vldmia r12!,{d8,d9,d10,d11,d12,d13,d14,d15}"),
    (Arm(0xf101_0200),        "setend BE"),
    (Arm(0x0000_80f4),        "strdeq r8,r9,[r0],-r4"),
    (Thumb16(0xbf0a),         "itet eq"),
    (Thumb32(0xeb40, 0x0001), "adc.w r0,r0,r1"),
    (Thumb32(0xf140, 0x0008), "adc r0,r0,#0x8"),
    //TODO attempt to shift left with overflow
    //(Thumb32(0xf7ff, 0xfffe),  "bl #0x10000"),
]),
(|v| v >= V7, &[
    (Arm(0xf460_408f), "vld4.32 {d20,d21,d22,d23},[r0]"),
    (Arm(0xf284_0650), "vmov.i32 q0,simdExpand(0x0,0x6,0x40)"),
    (Arm(0xf57f_f05b), "dmb ISH"),
]),
(|v| v >= V8, &[
    (Arm(0xf2be_0f11), "vcvt.s32.f32 d0,d1,#0x2"),
]),
//TODO
//(Arm(0xf420_060f), "vld1.8 {d0,d1,d2},[r0]"),
//(Arm(0xea27_c000), "b 0x9f0002"),
];

#[test]
fn arm() {
    for lib in LIBS.iter() {
        println!("arm {}", &lib.lib);
        let ld_lib = unsafe {
            libloading::Library::new(format!(
                "../target/debug/lib{}.so",
                &lib.lib
            ))
            .unwrap()
        };
        let parse_arm_fun: libloading::Symbol<
            fn(&'_ [u8], u32) -> Option<(u32, String)>,
        > = unsafe { ld_lib.get(b"parse_default\0").unwrap() };
        let parse_thumb_fun: Option<
            libloading::Symbol<fn(&'_ [u8], u32) -> Option<(u32, String)>>,
        > = lib
            .thumb
            .then(|| unsafe { ld_lib.get(b"parse_thumb\0").unwrap() });

        let instruction_and_parser = ARM
            .iter()
            .filter(|(check, _)| (check)(lib.version))
            .map(|(_, instructions)| instructions.iter())
            .flatten()
            .filter_map(|(instr, out)| match (instr.thumb_mode(), lib.thumb) {
                (true, true) => {
                    Some((parse_thumb_fun.as_ref().unwrap(), instr, out))
                }
                (true, false) => None,
                (false, _) => Some((&parse_arm_fun, instr, out)),
            });

        for (parse_fun, instruction, output) in instruction_and_parser {
            let token = instruction.to_tokens(lib.endian);
            let (_next_addr, parsed_output) =
                parse_fun(&token, 0).expect(&output);
            assert_eq!(&remove_spaces(&parsed_output), output);
        }
    }
}
